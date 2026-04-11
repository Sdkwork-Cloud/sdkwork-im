import {
  ArrowRight,
  GitBranch,
  Globe,
  Lock,
  Mail,
  QrCode,
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

import { ADMIN_ROUTE_PATHS, useAdminI18n } from 'sdkwork-craw-chat-admin-core';

type AuthMode = 'login' | 'register' | 'forgot';

const DEFAULT_LOGIN_STATUS = 'Authenticate to open the IM operator workspace.';
const SSO_NOTICE =
  'Use the operator email and password flow for IM admin access. External SSO remains disabled in this workspace.';
const DEV_ADMIN_CREDENTIALS = {
  email: 'admin@sdkwork.local',
  password: 'ChangeMe123!',
};

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
        title: 'Create operator access',
        description:
          'Request operator access and enter the IM operator workspace after an existing admin provisions your identity.',
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
        alternateLabel: 'Create access',
        alternatePath: ADMIN_ROUTE_PATHS.REGISTER,
        modeLabel: 'Recovery',
      };
    case 'login':
    default:
      return {
        title: 'Welcome back',
        description: 'Sign in to continue to your IM operator workspace.',
        submitLabel: 'Sign in',
        alternateLabel: 'Request access',
        alternatePath: ADMIN_ROUTE_PATHS.REGISTER,
        modeLabel: 'Sign in',
      };
  }
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
  const redirectTarget = resolveRedirectTarget(searchParams.get('redirect'));
  const copy = authCopy(mode);
  const [email, setEmail] = useState(
    import.meta.env.DEV && mode === 'login' ? DEV_ADMIN_CREDENTIALS.email : '',
  );
  const [password, setPassword] = useState(
    import.meta.env.DEV && mode === 'login' ? DEV_ADMIN_CREDENTIALS.password : '',
  );
  const [name, setName] = useState('');
  const [feedback, setFeedback] = useState('');
  const showDevCredentials = import.meta.env.DEV && mode === 'login';
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
      detail: t('Realtime posture, tenant coverage, and QR fallback remain available when password flows are under pressure.'),
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

    setEmail((current) => current.trim() || DEV_ADMIN_CREDENTIALS.email);
    setPassword((current) => current || DEV_ADMIN_CREDENTIALS.password);
  }, [showDevCredentials]);

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
          'Operator account requests stay inside the IM operator workspace. Ask an existing admin to provision {name} access from Users.',
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
              <QrCode className="h-8 w-8 text-white" />
            </div>
            <h2 className="mb-2 text-3xl font-black tracking-tight">{t('QR login')}</h2>
            <p className="mb-6 max-w-[280px] text-sm leading-6 text-zinc-300">
              {t(
                'Open the SDKWork app and scan this code to continue without typing credentials while the operator command post stays protected.',
              )}
            </p>

            <div className="mb-6 rounded-[28px] border border-white/10 bg-white/5 p-4 shadow-2xl backdrop-blur-sm">
              <div className="mb-4 flex items-center justify-between">
                <div>
                  <div className="text-xs font-semibold uppercase tracking-[0.22em] text-zinc-400">
                    {t('Operator command post')}
                  </div>
                  <div className="mt-1 text-sm font-semibold text-white">{t('Service continuity')}</div>
                </div>
                <div className="rounded-full border border-emerald-400/30 bg-emerald-400/15 px-2 py-1 text-[10px] font-semibold uppercase tracking-[0.18em] text-emerald-100">
                  {t('Live')}
                </div>
              </div>
              <div className="rounded-2xl bg-white p-4 shadow-xl">
                <div className="flex h-48 w-full items-center justify-center rounded-xl border-2 border-dashed border-zinc-300 bg-zinc-100">
                  <QrCode className="h-24 w-24 text-zinc-400" />
                </div>
              </div>
              <div className="mt-4 flex items-center gap-2 text-sm text-zinc-300">
                <Smartphone className="h-4 w-4" />
                <span>{t('Open app to scan')}</span>
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
                  {t(mode === 'login' ? 'Operator session' : 'IM operator workspace')}
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
                    placeholder={t('Workspace owner')}
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
                  placeholder={t('name@example.com')}
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
                  'Local dev credentials are prefilled: {email} / {password}.',
                  DEV_ADMIN_CREDENTIALS,
                )}
              </p>
            ) : null}

            {visibleFeedback ? (
              <p
                className={[
                  'mt-4 text-sm font-medium',
                  mode === 'login' ? 'text-rose-500' : 'text-zinc-500 dark:text-zinc-400',
                ].join(' ')}
              >
                {visibleFeedback}
              </p>
            ) : null}

            {mode === 'login' ? (
              <div className="mt-8">
                <div className="relative">
                  <div className="absolute inset-0 flex items-center">
                    <div className="w-full border-t border-zinc-200 dark:border-zinc-800" />
                  </div>
                  <div className="relative flex justify-center text-sm">
                    <span className="bg-white px-2 text-zinc-500 dark:bg-zinc-900">
                      {t('Continue with')}
                    </span>
                  </div>
                </div>

                <div className="mt-6 grid grid-cols-2 gap-3">
                  <Button
                    className="h-auto w-full rounded-xl border border-zinc-200 bg-white px-4 py-2.5 text-sm font-medium text-zinc-700 shadow-sm transition-colors hover:bg-zinc-50 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-300 dark:hover:bg-zinc-800"
                    onClick={() => setFeedback(t(SSO_NOTICE))}
                    type="button"
                    variant="outline"
                  >
                    <GitBranch className="h-5 w-5" />
                    {t('GitHub')}
                  </Button>
                  <Button
                    className="h-auto w-full rounded-xl border border-zinc-200 bg-white px-4 py-2.5 text-sm font-medium text-zinc-700 shadow-sm transition-colors hover:bg-zinc-50 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-300 dark:hover:bg-zinc-800"
                    onClick={() => setFeedback(t(SSO_NOTICE))}
                    type="button"
                    variant="outline"
                  >
                    <Globe className="h-5 w-5" />
                    {t('Google')}
                  </Button>
                </div>
              </div>
            ) : null}

            <div className="mt-8 text-center text-sm text-zinc-600 dark:text-zinc-400">
              {mode === 'login' ? (
                <>
                  {t('No account?')}{' '}
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
                  {t('Already have an account?')}{' '}
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
                  {t('Create account')}
                </Button>
              </div>
            ) : null}
          </div>
        </div>
      </div>
    </div>
  );
}
