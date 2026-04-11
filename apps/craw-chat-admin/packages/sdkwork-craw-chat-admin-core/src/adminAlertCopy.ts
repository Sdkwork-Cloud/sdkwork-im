type AdminAlertTranslationValues = Record<string, string | number>;

export type AdminAlertCopy = {
  text: string;
  values?: AdminAlertTranslationValues;
};

const EXHAUSTED_PROJECT_DETAIL_PATTERN = /^(\d+) projects have exhausted their traffic budget\.$/;
const MISSING_CREDENTIAL_DETAIL_PATTERN =
  /^(\d+) upstream connectors have no credential coverage\. Rotate or create credentials before opening live message traffic\.$/;

function formatAdminAlertValues(
  values: AdminAlertTranslationValues | undefined,
  formatNumber: (value: number) => string,
) {
  if (!values) {
    return undefined;
  }

  return Object.fromEntries(
    Object.entries(values).map(([key, value]) => [
      key,
      typeof value === 'number' ? formatNumber(value) : value,
    ]),
  );
}

export function resolveAdminAlertTitle(title: string) {
  return title.trim();
}

export function resolveAdminAlertDetailCopy(detail: string): AdminAlertCopy {
  const normalizedDetail = detail.trim();
  const exhaustedProjectMatch = normalizedDetail.match(EXHAUSTED_PROJECT_DETAIL_PATTERN);
  if (exhaustedProjectMatch) {
    return {
      text: '{count} projects have exhausted their traffic budget.',
      values: { count: Number(exhaustedProjectMatch[1]) },
    };
  }

  const missingCredentialMatch = normalizedDetail.match(MISSING_CREDENTIAL_DETAIL_PATTERN);
  if (missingCredentialMatch) {
    return {
      text: '{count} upstream connectors have no credential coverage. Rotate or create credentials before opening live message traffic.',
      values: { count: Number(missingCredentialMatch[1]) },
    };
  }

  return { text: normalizedDetail };
}

export function translateAdminAlertTitle(
  title: string,
  translate: (text: string) => string,
) {
  return translate(resolveAdminAlertTitle(title));
}

export function translateAdminAlertDetail(
  detail: string,
  translate: (text: string, values?: Record<string, unknown>) => string,
  formatNumber: (value: number) => string,
) {
  const copy = resolveAdminAlertDetailCopy(detail);
  return translate(copy.text, formatAdminAlertValues(copy.values, formatNumber));
}
