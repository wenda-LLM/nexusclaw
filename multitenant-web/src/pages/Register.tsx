import { useState, type FormEvent } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Mail, Lock, User } from 'lucide-react';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { useAuth } from '@/contexts/AuthContext';
import { t } from '@/lib/i18n';

export function RegisterPage() {
  const navigate = useNavigate();
  const { register } = useAuth();
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError('');

    if (password !== confirmPassword) {
      setError('两次输入的密码不一致');
      return;
    }

    if (password.length < 6) {
      setError('密码长度至少为 6 位');
      return;
    }

    setLoading(true);

    try {
      await register(email, password, name);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : '注册失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[var(--bg-base)] flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <div className="h-16 w-16 rounded-2xl bg-[var(--brand-500)] flex items-center justify-center text-white font-bold text-2xl mx-auto mb-4">
            ZC
          </div>
          <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('register.title')}</h1>
          <p className="text-[var(--text-secondary)] mt-2">创建 ZeroClaw 账号</p>
        </div>

        <div className="bg-[var(--bg-primary)] border border-[var(--border-base)] rounded-xl p-6">
          <form onSubmit={handleSubmit}>
            <div className="space-y-4">
              <Input
                label={t('register.name')}
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="你的昵称"
                icon={<User className="w-4 h-4" />}
                required
              />
              <Input
                label={t('register.email')}
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="your@email.com"
                icon={<Mail className="w-4 h-4" />}
                required
              />
              <Input
                label={t('register.password')}
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="至少 6 位"
                icon={<Lock className="w-4 h-4" />}
                required
              />
              <Input
                label={t('register.confirmPassword')}
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="再次输入密码"
                icon={<Lock className="w-4 h-4" />}
                required
              />
            </div>

            {error && (
              <p className="text-sm text-[var(--color-danger)] mt-3">{error}</p>
            )}

            <Button type="submit" loading={loading} className="w-full mt-6">
              {t('register.submit')}
            </Button>
          </form>

          <div className="mt-6 pt-6 border-t border-[var(--border-base)] text-center">
            <p className="text-sm text-[var(--text-secondary)]">
              {t('register.hasAccount')}{' '}
              <Link to="/login" className="text-[var(--color-primary)] hover:underline">
                {t('register.loginLink')}
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}