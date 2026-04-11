export function formatCompactNumber(value) {
  return new Intl.NumberFormat('zh-CN', {
    notation: 'compact',
    maximumFractionDigits: 1,
  }).format(value);
}

export function formatPercent(value) {
  return `${Math.round(value * 100)}%`;
}

export function formatMinutes(value) {
  if (value >= 60) {
    const hours = Math.floor(value / 60);
    const minutes = value % 60;
    return `${hours}h ${minutes}m`;
  }

  return `${value}m`;
}
