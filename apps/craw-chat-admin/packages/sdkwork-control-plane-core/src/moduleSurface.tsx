import type { ReactNode } from 'react';

export function AdminPageFrame({
  eyebrow,
  title,
  description,
  actions,
  children,
  rail,
}: {
  eyebrow: string;
  title: string;
  description: string;
  actions?: ReactNode;
  children: ReactNode;
  rail?: ReactNode;
}) {
  return (
    <div className="min-h-full space-y-6 px-6 py-7 md:px-8">
      <div className="flex flex-col gap-4 rounded-[32px] border border-[var(--admin-border-color)] bg-white/80 p-6 shadow-sm backdrop-blur dark:bg-zinc-950/70">
        <div className="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
          <div className="space-y-3">
            <div className="text-[11px] font-semibold uppercase tracking-[0.28em] text-[var(--admin-text-secondary)]">
              {eyebrow}
            </div>
            <div className="space-y-2">
              <h1 className="text-3xl font-semibold tracking-tight text-[var(--admin-text-primary)]">
                {title}
              </h1>
              <p className="max-w-3xl text-sm leading-6 text-[var(--admin-text-secondary)]">
                {description}
              </p>
            </div>
          </div>
          {actions ? <div className="flex flex-wrap gap-3">{actions}</div> : null}
        </div>
      </div>

      <div className={rail ? 'grid gap-6 xl:grid-cols-[minmax(0,1fr)_320px]' : 'space-y-6'}>
        <div className="space-y-6">{children}</div>
        {rail ? <div className="space-y-6">{rail}</div> : null}
      </div>
    </div>
  );
}

export function AdminMetricCard({
  label,
  value,
  detail,
}: {
  label: string;
  value: string;
  detail: string;
}) {
  return (
    <div className="rounded-[28px] border border-[var(--admin-border-color)] bg-white/80 p-5 shadow-sm dark:bg-zinc-950/70">
      <div className="text-sm font-medium text-[var(--admin-text-secondary)]">{label}</div>
      <div className="mt-3 text-3xl font-semibold tracking-tight text-[var(--admin-text-primary)]">
        {value}
      </div>
      <p className="mt-3 text-sm leading-6 text-[var(--admin-text-secondary)]">{detail}</p>
    </div>
  );
}

export function AdminSectionCard({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: ReactNode;
}) {
  return (
    <section className="rounded-[28px] border border-[var(--admin-border-color)] bg-white/80 p-5 shadow-sm dark:bg-zinc-950/70">
      <div className="mb-5 space-y-2">
        <h2 className="text-xl font-semibold tracking-tight text-[var(--admin-text-primary)]">
          {title}
        </h2>
        {description ? (
          <p className="text-sm leading-6 text-[var(--admin-text-secondary)]">{description}</p>
        ) : null}
      </div>
      {children}
    </section>
  );
}

export function AdminInsetCard({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <div
      className={`rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 ${className ?? ''}`}
    >
      {children}
    </div>
  );
}

export function AdminInsetSplitRow({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <AdminInsetCard
      className={`flex flex-col gap-3 md:flex-row md:items-center md:justify-between ${className ?? ''}`}
    >
      {children}
    </AdminInsetCard>
  );
}

export function AdminGuidanceList({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <div className={`space-y-3 text-sm text-[var(--admin-text-secondary)] ${className ?? ''}`}>
      {children}
    </div>
  );
}

export function AdminEmptyState({
  title,
  detail,
  className,
}: {
  title: string;
  detail: string;
  className?: string;
}) {
  return (
    <div
      className={`rounded-3xl border border-dashed border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/40 p-5 ${className ?? ''}`}
    >
      <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{title}</div>
      <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{detail}</div>
    </div>
  );
}

export function AdminActionChip({
  label,
  tone = 'default',
}: {
  label: string;
  tone?: 'default' | 'success' | 'warning';
}) {
  const toneClassName =
    tone === 'success'
      ? 'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-900/60 dark:bg-emerald-950/50 dark:text-emerald-200'
      : tone === 'warning'
        ? 'border-amber-200 bg-amber-50 text-amber-700 dark:border-amber-900/60 dark:bg-amber-950/50 dark:text-amber-200'
        : 'border-[var(--admin-border-color)] bg-white/70 text-[var(--admin-text-primary)] dark:bg-zinc-900/80';

  return (
    <span
      className={`inline-flex items-center rounded-full border px-3 py-1.5 text-sm font-medium ${toneClassName}`}
    >
      {label}
    </span>
  );
}
