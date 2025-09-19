(() => {
  const CONFIG = {
    concurrency: 10,
    missStreak: 3,
    maxIndex: 9999,
  };

  const zeroPad = (number, width) => String(number).padStart(width, '0');

  const isBatchable = (url) => /(\{(\d+)\.\.(\d+)\}|\[(\d+)\-(\d+)\]|0\*|\*)/.test(url);

  const headOk = async (url) => await ferrix.api.http.head(url);

  const parsePattern = (url) => {
    const rangePattern = /^(.*)\{(\d+)\.\.(\d+)\}(.*)$/;
    const bracketPattern = /^(.*)\[(\d+)\-(\d+)\](.*)$/;
    const sequentialPattern = /^(.*?)(0*)\*(.*)$/;

    const rangeMatch = url.match(rangePattern) || url.match(bracketPattern);
    if (rangeMatch) {
      const [, prefix, startStr, endStr, suffix] = rangeMatch;
      const start = parseInt(startStr, 10);
      const end = parseInt(endStr, 10);
      const padLength = Math.max(startStr.length, endStr.length);

      return {
        type: 'range',
        prefix,
        start,
        end,
        padLength,
        suffix,
      };
    }

    const sequentialMatch = url.match(sequentialPattern);
    if (sequentialMatch) {
      const [, prefix, zeros, suffix] = sequentialMatch;
      const padLength = zeros ? zeros.length + 1 : 1;

      return {
        type: 'sequential',
        prefix,
        suffix,
        start: 1,
        padLength,
      };
    }

    return null;
  };

  const probeRange = async (prefix, suffix, start, end, padLength) => {
    const foundUrls = [];

    for (let current = start; current <= end; ) {
      const batch = [];

      for (let i = 0; i < CONFIG.concurrency && current <= end; i++, current++) {
        const url = `${prefix}${zeroPad(current, padLength)}${suffix}`;

        batch.push(
          headOk(url).then((isValid) => {
            if (isValid) foundUrls.push(url);
          }),
        );
      }

      await Promise.allSettled(batch);
    }

    return foundUrls;
  };

  const probeSequential = async (prefix, suffix, start, padLength) => {
    const foundUrls = [];
    let current = start;
    let consecutiveMisses = 0;

    while (current <= CONFIG.maxIndex && consecutiveMisses < CONFIG.missStreak) {
      const batch = [];

      for (let i = 0; i < CONFIG.concurrency && current <= CONFIG.maxIndex; i++, current++) {
        const url = `${prefix}${zeroPad(current, padLength)}${suffix}`;

        batch.push(
          headOk(url).then((isValid) => {
            if (isValid) {
              foundUrls.push(url);
              consecutiveMisses = 0;
            } else {
              consecutiveMisses++;
            }
          }),
        );
      }

      await Promise.allSettled(batch);
    }

    return foundUrls;
  };

  ferrix.on('new-download', async (event, payload) => {
    const inputUrl = String(payload.url || '').trim();

    if (!isBatchable(inputUrl)) return;

    try {
      const pattern = parsePattern(inputUrl);
      if (!pattern) return;

      let discoveredUrls = [];

      if (pattern.type === 'range') {
        discoveredUrls = await probeRange(
          pattern.prefix,
          pattern.suffix,
          pattern.start,
          pattern.end,
          pattern.padLength,
        );
      } else {
        discoveredUrls = await probeSequential(
          pattern.prefix,
          pattern.suffix,
          pattern.start,
          pattern.padLength,
        );

        if (discoveredUrls.length === 0) {
          discoveredUrls = await probeSequential(
            pattern.prefix,
            pattern.suffix,
            0,
            pattern.padLength,
          );
        }
      }

      event.mutate({ ...payload, url: discoveredUrls.join('\n') });
    } catch (error) {
      console.warn('Ferrix batch plugin error:', error);
    }
  });
})();
