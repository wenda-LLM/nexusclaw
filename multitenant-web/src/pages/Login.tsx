import { useState, type FormEvent } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Mail, Lock } from 'lucide-react';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { useAuth } from '@/contexts/AuthContext';
import { t } from '@/lib/i18n';

export function LoginPage() {
  const navigate = useNavigate();
  const { login, pair } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [showPairing, setShowPairing] = useState(false);
  const [pairCode, setPairCode] = useState('');

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      await login(email, password);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : '登录失败');
    } finally {
      setLoading(false);
    }
  };

  const handlePair = async (e: FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      await pair(pairCode);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : '配对失败');
    } finally {
      setLoading(false);
    }
  };

  if (showPairing) {
    return (
      <div className="min-h-screen bg-[var(--bg-base)] flex items-center justify-center p-4">
        <div className="w-full max-w-md">
          <div className="text-center mb-8">
            <div className="h-16 w-16 rounded-2xl bg-[var(--brand-500)] flex items-center justify-center text-white font-bold text-2xl mx-auto mb-4">
              ZC
            </div>
            <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('login.pairing')}</h1>
            <p className="text-[var(--text-secondary)] mt-2">输入终端显示的配对码</p>
          </div>

          <div className="bg-[var(--bg-primary)] border border-[var(--border-base)] rounded-xl p-6">
            <form onSubmit={handlePair}>
              <Input
                label="配对码"
                value={pairCode}
                onChange={(e) => setPairCode(e.target.value)}
                placeholder="6位数字"
                maxLength={6}
                className="text-center text-2xl tracking-widest"
                autoFocus
              />
              
              {error && (
                <p className="text-sm text-[var(--color-danger)] mt-3 text-center">{error}</p>
              )}

              <div className="mt-6 flex gap-3">
                <Button
                  type="button"
                  variant="secondary"
                  className="flex-1"
                  onClick={() => setShowPairing(false)}
                >
                  返回
                </Button>
                <Button type="submit" loading={loading} disabled={pairCode.length < 6} className="flex-1">
                  配对
                </Button>
              </div>
            </form>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[var(--bg-base)] flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <div className="h-16 w-16 rounded-2xl bg-[var(--brand-500)] flex items-center justify-center text-white font-bold text-2xl mx-auto mb-4">
            ZC
          </div>
          <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('login.title')}</h1>
          <p className="text-[var(--text-secondary)] mt-2">登录到 ZeroClaw 管理平台</p>
        </div>

        <div className="bg-[var(--bg-primary)] border border-[var(--border-base)] rounded-xl p-6">
          <form onSubmit={handleSubmit}>
            <div className="space-y-4">
              <Input
                label={t('login.email')}
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="your@email.com"
                icon={<Mail className="w-4 h-4" />}
                required
              />
              <Input
                label={t('login.password')}
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="••••••••"
                icon={<Lock className="w-4 h-4" />}
                required
              />
            </div>

            {error && (
              <p className="text-sm text-[var(--color-danger)] mt-3">{error}</p>
            )}

            <Button type="submit" loading={loading} className="w-full mt-6">
              {t('login.submit')}
            </Button>

            <div className="mt-4 text-center">
              <button
                type="button"
                onClick={() => setShowPairing(true)}
                className="text-sm text-[var(--color-primary)] hover:underline"
              >
                {t('login.pairing')}
              </button>
            </div>
          </form>

          <div className="mt-6 pt-6 border-t border-[var(--border-base)] text-center">
            <p className="text-sm text-[var(--text-secondary)]">
              {t('login.noAccount')}{' '}
              <Link to="/register" className="text-[var(--color-primary)] hover:underline">
                {t('login.registerLink')}
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}