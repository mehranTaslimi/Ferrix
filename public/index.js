const { on } = ferrix;

on('new-download', async (ev, payload) => {
  ev.mutate({
    ...payload,
    url: 'https://releases.ubuntu.com/24.04.3/ubuntu-24.04.3-desktop-amd64.iso',
  });
});
