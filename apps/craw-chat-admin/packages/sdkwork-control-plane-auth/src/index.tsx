import {
  ArrowRight,
  GitBranch,
  Globe,
  Lock,
  Mail,
  ShieldCheck,
  Smartphone,
  TimerReset,
  User,
} from 'lucide-react';
import {
  useEffect,
  useState,
  type ComponentPropsWithoutRef,
  type ComponentType,
  type FormEvent,
  type ReactNode,
} from 'react';
import { Navigate, useLocation, useNavigate, useSearchParams } from 'react-router-dom';
import { Badge, Button, Input, Label } from '@sdkwork/ui-pc-react';

import { ADMIN_ROUTE_PATHS, useAdminI18n } from 'sdkwork-control-plane-core';

type AuthMode = 'login' | 'register' | 'forgot';

const DEFAULT_LOGIN_STATUS = 'Authenticate to open the control-plane workspace.';

type AuthBadgeProps = {
  children?: ReactNode;
  className?: string;
  variant?: 'default' | 'secondary' | 'outline' | 'danger' | 'success' | 'warning';
};

const AuthBadge = Badge as unknown as ComponentType<AuthBadgeProps>;

function resolveAuthMode(pathname: string): AuthMode {
  if (pathname === ADMIN_ROUTE_PATHS.REGISTER) {
    return 'register';
  }

  if (pathname === ADMIN_ROUTE_PATHS.FORGOT_PASSWORD) {
    return 'forgot';
  }

  return 'login';
}

function resolveRedirectTarget(rawTarget: string | null) {
  if (!rawTarget || !rawTarget.startsWith('/')) {
    return ADMIN_ROUTE_PATHS.OVERVIEW;
  }

  if (
    rawTarget === ADMIN_ROUTE_PATHS.ROOT ||
    rawTarget === ADMIN_ROUTE_PATHS.AUTH ||
    rawTarget === ADMIN_ROUTE_PATHS.LOGIN ||
    rawTarget === ADMIN_ROUTE_PATHS.REGISTER ||
    rawTarget === ADMIN_ROUTE_PATHS.FORGOT_PASSWORD
  ) {
    return ADMIN_ROUTE_PATHS.OVERVIEW;
  }

  return rawTarget;
}

function authCopy(mode: AuthMode) {
  switch (mode) {
    case 'register':
      return {
        title: 'Request operator access',
        description:
          'Request operator access and enter the control-plane workspace after an existing admin provisions your identity.',
        submitLabel: 'Request access',
        alternateLabel: 'Sign in',
        alternatePath: ADMIN_ROUTE_PATHS.LOGIN,
        modeLabel: 'Request access',
      };
    case 'forgot':
      return {
        title: 'Recover access',
        description:
          'Password reset links are not enabled for this workspace yet. Continue back to sign in with your operator email.',
        submitLabel: 'Back to login',
        alternateLabel: 'Request access',
        alternatePath: ADMIN_ROUTE_PATHS.REGISTER,
        modeLabel: 'Recovery',
      };
    case 'login':
    default:
      return {
        title: 'Welcome back',
        description: 'Sign in to continue to your control-plane workspace.',
        submitLabel: 'Sign in',
        alternateLabel: 'Request access',
        alternatePath: ADMIN_ROUTE_PATHS.REGISTER,
        modeLabel: 'Sign in',
      };
  }
}

function resolveLoginFeedbackClassName(status: string, loading: boolean) {
  const normalized = status.toLowerCase();

  if (
    loading ||
    normalized.includes('signed out') ||
    normalized.includes('synchronized') ||
    normalized.includes('refreshing live control-plane data') ||
    normalized.includes('establishing operator session') ||
    normalized.includes('operator session established')
  ) {
    return 'text-zinc-500 dark:text-zinc-400';
  }

  return 'text-rose-500';
}

function AuthTextInput({
  icon,
  inputClassName,
  style,
  type,
  ...props
}: Omit<ComponentPropsWithoutRef<'input'>, 'className'> & {
  icon: ReactNode;
  inputClassName?: string;
}) {
  return (
    <div className="relative block w-full">
      <span className="pointer-events-none absolute left-4 top-1/2 flex h-5 w-5 -translate-y-1/2 items-center justify-center text-zinc-400 dark:text-zinc-500">
        {icon}
      </span>
      <Input
        {...props}
        className={['h-10 pr-3', inputClassName].filter(Boolean).join(' ')}
        style={{ ...style, paddingLeft: '2.75rem' }}
        type={type ?? 'text'}
      />
    </div>
  );
}

export function AdminLoginPage({
  status,
  loading,
  isAuthenticated,
  onLogin,
}: {
  status: string;
  loading: boolean;
  isAuthenticated: boolean;
  onLogin: (input: { email: string; password: string }) => Promise<void>;
}) {
  const { t } = useAdminI18n();
  const navigate = useNavigate();
  const location = useLocation();
  const [searchParams] = useSearchParams();
  const mode = resolveAuthMode(location.pathname);
  const rawDevAdminPrefillEmail = import.meta.env.VITE_ADMIN_SANDBOX_EMAIL;
  const rawDevAdminPrefillPassword = import.meta.env.VITE_ADMIN_SANDBOX_PASSWORD;
  const devAdminPrefillEmail =
    import.meta.env.DEV && mode === 'login' && typeof rawDevAdminPrefillEmail === 'string'
      ? rawDevAdminPrefillEmail.trim()
      : '';
  const devAdminPrefillPassword =
    import.meta.env.DEV && mode === 'login' && typeof rawDevAdminPrefillPassword === 'string'
      ? rawDevAdminPrefillPassword
      : '';
  const showDevCredentials =
    devAdminPrefillEmail.length > 0 && devAdminPrefillPassword.length > 0;
  const redirectTarget = resolveRedirectTarget(searchParams.get('redirect'));
  const copy = authCopy(mode);
  const [email, setEmail] = useState(
    showDevCredentials ? devAdminPrefillEmail : '',
  );
  const [password, setPassword] = useState(
    showDevCredentials ? devAdminPrefillPassword : '',
  );
  const [name, setName] = useState('');
  const [feedback, setFeedback] = useState('');
  const operatorSignals = [
    {
      id: 'handoff',
      title: t('Shift handoff'),
      detail: t('Queue ownership, frozen conversations, and escalation context stay visible across operator shifts.'),
      icon: TimerReset,
    },
    {
      id: 'trust',
      title: t('Trust and safety'),
      detail: t('Step-up enforced for recovery, evidence export, and abuse intervention before access is widened.'),
      icon: ShieldCheck,
    },
    {
      id: 'continuity',
      title: t('Service continuity'),
      detail: t('Realtime posture and tenant coverage remain visible while sign-in stays on the verified password-first path.'),
      icon: Smartphone,
    },
  ];

  useEffect(() => {
    const nextEmail = searchParams.get('email');
    if (nextEmail) {
      setEmail(nextEmail);
    }
  }, [searchParams]);

  useEffect(() => {
    setFeedback('');
  }, [mode]);

  useEffect(() => {
    if (!showDevCredentials) {
      return;
    }

    setEmail((current) => current.trim() || devAdminPrefillEmail);
    setPassword((current) => current || devAdminPrefillPassword);
  }, [devAdminPrefillEmail, devAdminPrefillPassword, showDevCredentials]);

  function withRedirect(pathname: string, extra: Record<string, string> = {}) {
    const params = new URLSearchParams();

    if (redirectTarget !== ADMIN_ROUTE_PATHS.OVERVIEW) {
      params.set('redirect', redirectTarget);
    }

    Object.entries(extra).forEach(([key, value]) => {
      if (value) {
        params.set(key, value);
      }
    });

    const queryString = params.toString();
    return queryString ? `${pathname}?${queryString}` : pathname;
  }

  const visibleFeedback =
    feedback || (mode === 'login' && status && status !== DEFAULT_LOGIN_STATUS ? t(status) : '');

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (mode === 'login') {
      await onLogin({ email, password });
      return;
    }

    if (mode === 'register') {
      setFeedback(
        t(
          'Operator account requests stay inside the control-plane workspace. Ask an existing admin to review and provision {name} access from Identity.',
          { name: name.trim() || email.trim() || t('your') },
        ),
      );
      return;
    }

    navigate(withRedirect(ADMIN_ROUTE_PATHS.LOGIN, { email }), { replace: true });
  }

  if (isAuthenticated) {
    return <Navigate to={redirectTarget} replace />;
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 p-4 dark:bg-zinc-950 sm:p-8">
      <div className="flex w-full max-w-4xl flex-col overflow-hidden rounded-3xl bg-white shadow-2xl dark:bg-zinc-900 md:flex-row">
        <div className="relative flex w-full flex-col items-center justify-center overflow-hidden bg-zinc-900 p-8 text-white dark:bg-black md:w-2/5">
          <div className="absolute inset-0 bg-gradient-to-br from-[var(--sdk-color-brand-primary)]/25 to-transparent" />
          <div className="relative z-10 flex w-full max-w-sm flex-col text-left">
            <div className="mb-6 flex items-center gap-3">
              <AuthBadge
                className="border-white/20 bg-white/10 text-white"
                variant="outline"
              >
                {t('Craw Chat Admin')}
              </AuthBadge>
              <AuthBadge
                className="border-emerald-400/30 bg-emerald-400/15 text-emerald-100"
                variant="outline"
              >
                {t('Step-up enforced')}
              </AuthBadge>
            </div>

            <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-2xl bg-[var(--sdk-color-brand-primary)] shadow-lg">
              <ShieldCheck className="h-8 w-8 text-white" />
            </div>
            <h2 className="mb-2 text-3xl font-black tracking-tight">{t('Protected operator access')}</h2>
            <p className="mb-6 max-w-[280px] text-sm leading-6 text-zinc-300">
              {t('This workspace currently uses operator email and password as the only live sign-in path.')}
            </p>

            <div className="mb-6 rounded-[28px] border border-white/10 bg-white/5 p-4 shadow-2xl backdrop-blur-sm">
              <div className="mb-4 flex items-center justify-between">
                <div>
                  <div className="text-xs font-semibold uppercase tracking-[0.22em] text-zinc-400">
                    {t('Active sign-in path')}
                  </div>
                  <div className="mt-1 text-sm font-semibold text-white">{t('Password-first access')}</div>
                </div>
                <div className="rounded-full border border-emerald-400/30 bg-emerald-400/15 px-2 py-1 text-[10px] font-semibold uppercase tracking-[0.18em] text-emerald-100">
                  {t('Live')}
                </div>
              </div>
              <div className="rounded-2xl border border-white/10 bg-black/20 p-4">
                <div className="text-sm font-semibold text-white">{t('Email and password sign-in is active for this workspace.')}</div>
              </div>
              <div className="mt-4 space-y-2 text-sm leading-6 text-zinc-300">
                <div>{t('Recovery and evidence actions still require step-up review before access expands.')}</div>
                <div>{t('Mobile handoff will appear here after the workspace publishes a verified sign-in bridge.')}</div>
              </div>
            </div>

            <div className="mb-6 rounded-[28px] border border-white/10 bg-white/5 p-4 shadow-2xl backdrop-blur-sm">
              <div className="mb-4">
                <div className="text-xs font-semibold uppercase tracking-[0.22em] text-zinc-400">
                  {t('Identity provider availability')}
                </div>
                <div className="mt-1 text-sm leading-6 text-zinc-300">
                  {t('External identity providers remain disabled until workspace policy enables a live provider.')}
                </div>
              </div>
              <div className="space-y-3">
                <div className="rounded-2xl border border-white/10 bg-black/20 p-4">
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex items-start gap-3">
                      <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-white/10 text-white">
                        <GitBranch className="h-5 w-5" />
                      </div>
                      <div>
                        <div className="text-sm font-semibold text-white">{t('GitHub')}</div>
                        <div className="mt-1 text-sm leading-6 text-zinc-300">
                          {t('GitHub sign-in is disabled until a live provider policy is configured.')}
                        </div>
                      </div>
                    </div>
                    <div className="rounded-full border border-white/10 bg-white/10 px-2 py-1 text-[10px] font-semibold uppercase tracking-[0.18em] text-zinc-200">
                      {t('Disabled')}
                    </div>
                  </div>
                </div>
                <div className="rounded-2xl border border-white/10 bg-black/20 p-4">
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex items-start gap-3">
                      <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-white/10 text-white">
                        <Globe className="h-5 w-5" />
                      </div>
                      <div>
                        <div className="text-sm font-semibold text-white">{t('Google')}</div>
                        <div className="mt-1 text-sm leading-6 text-zinc-300">
                          {t('Google sign-in is disabled until a live provider policy is configured.')}
                        </div>
                      </div>
                    </div>
                    <div className="rounded-full border border-white/10 bg-white/10 px-2 py-1 text-[10px] font-semibold uppercase tracking-[0.18em] text-zinc-200">
                      {t('Disabled')}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div className="space-y-3">
              {operatorSignals.map((signal) => (
                <div
                  className="rounded-2xl border border-white/10 bg-white/5 p-4 backdrop-blur-sm"
                  key={signal.id}
                >
                  <div className="flex items-center gap-3">
                    <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-white/10 text-white">
                      <signal.icon className="h-5 w-5" />
                    </div>
                    <div>
                      <div className="text-sm font-semibold text-white">{signal.title}</div>
                      <div className="mt-1 text-sm leading-6 text-zinc-300">{signal.detail}</div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="w-full p-8 md:w-3/5 md:p-12">
          <div className="mx-auto max-w-md">
            <div className="mb-8">
              <div className="mb-3 flex items-center gap-3">
                <AuthBadge variant="secondary">
                  {t(mode === 'login' ? 'Operator session' : 'Control-plane workspace')}
                </AuthBadge>
                <AuthBadge variant="outline">{t(copy.modeLabel)}</AuthBadge>
              </div>
              <h1 className="mb-2 text-3xl font-black tracking-tight text-zinc-900 dark:text-white">
                {t(copy.title)}
              </h1>
              <p className="text-zinc-500 dark:text-zinc-400">{t(copy.description)}</p>
            </div>

            <form className="space-y-5" onSubmit={handleSubmit}>
              {mode === 'register' ? (
                <div>
                  <Label className="mb-1.5 block text-zinc-700 dark:text-zinc-300">
                    {t('Name')}
                  </Label>
                  <AuthTextInput
                    autoComplete="name"
                    icon={<User className="h-5 w-5" />}
                    onChange={(event) => setName(event.target.value)}
                    placeholder={t('Operations lead')}
                    required
                    type="text"
                    value={name}
                  />
                </div>
              ) : null}

              <div>
                <Label className="mb-1.5 block text-zinc-700 dark:text-zinc-300">
                  {t('Email')}
                </Label>
                <AuthTextInput
                  autoComplete="email"
                  icon={<Mail className="h-5 w-5" />}
                  onChange={(event) => setEmail(event.target.value)}
                  placeholder={t('ops@workspace.example')}
                  required
                  type="email"
                  value={email}
                />
              </div>

              {mode !== 'forgot' ? (
                <div>
                  <div className="mb-1.5 flex items-center justify-between">
                    <Label className="text-zinc-700 dark:text-zinc-300">{t('Password')}</Label>
                    {mode === 'login' ? (
                      <Button
                        className="h-auto rounded-none p-0 text-sm font-medium text-[var(--sdk-color-brand-primary)] shadow-none hover:bg-transparent hover:text-[var(--sdk-color-brand-primary-hover)]"
                        onClick={() =>
                          navigate(withRedirect(ADMIN_ROUTE_PATHS.FORGOT_PASSWORD, { email }))
                        }
                        type="button"
                        variant="ghost"
                      >
                        {t('Forgot password?')}
                      </Button>
                    ) : null}
                  </div>
                  <AuthTextInput
                    autoComplete={mode === 'register' ? 'new-password' : 'current-password'}
                    icon={<Lock className="h-5 w-5" />}
                    onChange={(event) => setPassword(event.target.value)}
                    placeholder={
                      mode === 'register' ? t('Create a password') : t('Enter your password')
                    }
                    required
                    type="password"
                    value={password}
                  />
                </div>
              ) : null}

              <Button
                className="h-auto w-full rounded-xl py-3 font-bold shadow-sm"
                loading={mode === 'login' ? loading : false}
                type="submit"
                variant="primary"
              >
                {t(mode === 'login' && loading ? 'Signing In...' : copy.submitLabel)}
                <ArrowRight className="h-4 w-4" />
              </Button>
            </form>

            {showDevCredentials ? (
              <p className="mt-4 text-sm font-medium text-zinc-500 dark:text-zinc-400">
                {t(
                  'Local dev login is prefilled from VITE_ADMIN_SANDBOX_EMAIL and VITE_ADMIN_SANDBOX_PASSWORD.',
                )}
              </p>
            ) : null}

            {visibleFeedback ? (
              <p
                className={[
                  'mt-4 text-sm font-medium',
                  mode === 'login'
                    ? resolveLoginFeedbackClassName(status, loading)
                    : 'text-zinc-500 dark:text-zinc-400',
                ].join(' ')}
              >
                {visibleFeedback}
              </p>
            ) : null}

            <div className="mt-8 text-center text-sm text-zinc-600 dark:text-zinc-400">
              {mode === 'login' ? (
                <>
                  {t('Need access?')}{' '}
                  <Button
                    className="h-auto rounded-none p-0 font-bold text-[var(--sdk-color-brand-primary)] shadow-none hover:bg-transparent hover:text-[var(--sdk-color-brand-primary-hover)]"
                    onClick={() => navigate(withRedirect(copy.alternatePath))}
                    type="button"
                    variant="ghost"
                  >
                    {t(copy.alternateLabel)}
                  </Button>
                </>
              ) : mode === 'register' ? (
                <>
                  {t('Already provisioned?')}{' '}
                  <Button
                    className="h-auto rounded-none p-0 font-bold text-[var(--sdk-color-brand-primary)] shadow-none hover:bg-transparent hover:text-[var(--sdk-color-brand-primary-hover)]"
                    onClick={() => navigate(withRedirect(copy.alternatePath))}
                    type="button"
                    variant="ghost"
                  >
                    {t(copy.alternateLabel)}
                  </Button>
                </>
              ) : (
                <Button
                  className="mx-auto h-auto gap-1 rounded-none p-0 font-bold text-[var(--sdk-color-brand-primary)] shadow-none hover:bg-transparent hover:text-[var(--sdk-color-brand-primary-hover)]"
                  onClick={() => navigate(withRedirect(ADMIN_ROUTE_PATHS.LOGIN, { email }))}
                  type="button"
                  variant="ghost"
                >
                  <ArrowRight className="h-4 w-4 rotate-180" />
                  {t('Back to login')}
                </Button>
              )}
            </div>

            {mode === 'forgot' ? (
              <div className="mt-4 text-center">
                <Button
                  className="h-auto rounded-none p-0 text-sm font-medium text-[var(--sdk-color-brand-primary)] shadow-none hover:bg-transparent hover:text-[var(--sdk-color-brand-primary-hover)]"
                  onClick={() => navigate(withRedirect(ADMIN_ROUTE_PATHS.REGISTER))}
                  type="button"
                  variant="ghost"
                >
                  {t('Request access')}
                </Button>
              </div>
            ) : null}
          </div>
        </div>
      </div>
    </div>
  );
}
