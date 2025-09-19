import Ferrix from './ferrix';

declare global {
  interface Window {
    ferrix: Ferrix;
  }
}

(() => {
  if (typeof window === 'undefined') return;
  window.ferrix = new Ferrix();
})();
