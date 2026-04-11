function assertPortalRecord(name, value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new TypeError(`${name} must be an object.`);
  }

  return value;
}

function assertNonEmptyString(name, value) {
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new TypeError(`${name} must be a non-empty string.`);
  }

  return value;
}

export function assertPortalText(name, value) {
  return assertNonEmptyString(name, value);
}

export function assertPortalHeroSnapshot(name, hero) {
  const heroRecord = assertPortalRecord(`${name} hero`, hero);
  assertNonEmptyString(`${name} hero.title`, heroRecord.title);
  assertNonEmptyString(`${name} hero.description`, heroRecord.description);
  return heroRecord;
}

export function assertPortalDisplayValue(name, value) {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }

  return assertNonEmptyString(name, value);
}

export function assertPortalSnapshotArray(name, value) {
  if (!Array.isArray(value)) {
    throw new TypeError(`${name} must be an array.`);
  }

  return value;
}

export function assertPortalSnapshotRecord(name, value) {
  return assertPortalRecord(name, value);
}

function assertPortalArrayItem(name, value) {
  return assertPortalRecord(name, value);
}

export function assertPortalProgressItems(name, items) {
  assertPortalSnapshotArray(name, items).forEach((item, index) => {
    const record = assertPortalArrayItem(`${name}[${index}]`, item);
    assertNonEmptyString(`${name}[${index}].label`, record.label);
    assertPortalDisplayValue(`${name}[${index}].value`, record.value);

    if (typeof record.percent !== 'number' || !Number.isFinite(record.percent)) {
      throw new TypeError(`${name}[${index}].percent must be a finite number.`);
    }
  });
}

export function assertPortalClusterItems(name, items) {
  assertPortalSnapshotArray(name, items).forEach((item, index) => {
    const record = assertPortalArrayItem(`${name}[${index}]`, item);
    assertNonEmptyString(`${name}[${index}].label`, record.label);
    assertPortalDisplayValue(`${name}[${index}].value`, record.value);
    assertNonEmptyString(`${name}[${index}].status`, record.status);
  });
}

export function assertPortalBulletItems(name, items) {
  assertPortalSnapshotArray(name, items).forEach((item, index) => {
    const record = assertPortalArrayItem(`${name}[${index}]`, item);
    assertNonEmptyString(`${name}[${index}].title`, record.title);
  });
}

export function assertPortalTableItems(name, items, fields) {
  assertPortalSnapshotArray(name, items).forEach((item, index) => {
    const record = assertPortalArrayItem(`${name}[${index}]`, item);

    for (const field of fields) {
      assertPortalDisplayValue(`${name}[${index}].${field}`, record[field]);
    }
  });
}
