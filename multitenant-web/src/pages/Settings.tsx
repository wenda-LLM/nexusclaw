import { useState, type FormEvent } from 'react';
import { User, Lock, Palette, Save, CheckCircle } from 'lucide-react';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { Card } from '@/components/common/Card';
import { useAuth } from '@/contexts/AuthContext';
import { useTheme } from '@/contexts/ThemeContext';
import { t } from '@/lib/i18n';
import * as api from '@/lib/api';

export function SettingsPage() {
  const { user, updateUser } = useAuth();
  const { theme, setTheme } = useTheme();
  const [activeTab, setActiveTab] = useState<'profile' | 'password' | 'theme'>('profile');
  const [name, setName] = useState(user?.name || '');
  const [oldPassword, setOldPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState('');

  const handleUpdateProfile = async (e: FormEvent) => {
    e.preventDefault();
    setSaving(true);
    setMessage('');

    try {
      const updatedUser = await api.updateProfile({ name });
      updateUser(updatedUser);
      setMessage('个人信息已更新');
    } catch (e) {
      setMessage(e instanceof Error ? e.message : '更新失败');
    } finally {
      setSaving(false);
    }
  };

  const handleChangePassword = async (e: FormEvent) => {
    e.preventDefault();
    
    if (newPassword !== confirmPassword) {
      setMessage('两次输入的密码不一致');
      return;
    }

    if (newPassword.length < 6) {
      setMessage('密码长度至少为 6 位');
      return;
    }

    setSaving(true);
    setMessage('');

    try {
      await api.changePassword(oldPassword, newPassword);
      setMessage('密码已修改');
      setOldPassword('');
      setNewPassword('');
      setConfirmPassword('');
    } catch (e) {
      setMessage(e instanceof Error ? e.message : '修改密码失败');
    } finally {
      setSaving(false);
    }
  };

  const tabs = [
    { id: 'profile', label: t('settings.profile'), icon: User },
    { id: 'password', label: t('settings.password'), icon: Lock },
    { id: 'theme', label: t('settings.theme'), icon: Palette },
  ] as const;

  return (
    <div className="max-w-3xl">
      <h1 className="text-2xl font-bold text-[var(--text-primary)] mb-6">{t('nav.settings')}</h1>

      <div className="flex gap-6">
        <div className="w-48 flex-shrink-0">
          <Card className="p-2">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`w-full flex items-center gap-2 px-3 py-2 rounded-lg text-sm transition-colors ${
                  activeTab === tab.id
                    ? 'bg-[var(--color-primary)] text-white'
                    : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'
                }`}
              >
                <tab.icon className="w-4 h-4" />
                {tab.label}
              </button>
            ))}
          </Card>
        </div>

        <div className="flex-1">
          <Card>
            {activeTab === 'profile' && (
              <form onSubmit={handleUpdateProfile}>
                <h2 className="text-lg font-semibold text-[var(--text-primary)] mb-4">{t('settings.profile')}</h2>
                <div className="space-y-4">
                  <Input
                    label={t('register.name')}
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                  />
                  <Input
                    label={t('login.email')}
                    value={user?.email || ''}
                    disabled
                  />
                </div>
                <div className="mt-6 flex items-center gap-3">
                  <Button type="submit" loading={saving}>
                    <Save className="w-4 h-4 mr-2" />
                    {t('common.save')}
                  </Button>
                  {message && (
                    <span className={`text-sm ${message.includes('已') ? 'text-[var(--color-success)]' : 'text-[var(--color-danger)]'}`}>
                      {message}
                    </span>
                  )}
                </div>
              </form>
            )}

            {activeTab === 'password' && (
              <form onSubmit={handleChangePassword}>
                <h2 className="text-lg font-semibold text-[var(--text-primary)] mb-4">{t('settings.password')}</h2>
                <div className="space-y-4">
                  <Input
                    label="当前密码"
                    type="password"
                    value={oldPassword}
                    onChange={(e) => setOldPassword(e.target.value)}
                    required
                  />
                  <Input
                    label="新密码"
                    type="password"
                    value={newPassword}
                    onChange={(e) => setNewPassword(e.target.value)}
                    required
                  />
                  <Input
                    label="确认新密码"
                    type="password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    required
                  />
                </div>
                <div className="mt-6 flex items-center gap-3">
                  <Button type="submit" loading={saving}>
                    <Save className="w-4 h-4 mr-2" />
                    {t('common.save')}
                  </Button>
                  {message && (
                    <span className={`text-sm ${message.includes('已') ? 'text-[var(--color-success)]' : 'text-[var(--color-danger)]'}`}>
                      {message}
                    </span>
                  )}
                </div>
              </form>
            )}

            {activeTab === 'theme' && (
              <div>
                <h2 className="text-lg font-semibold text-[var(--text-primary)] mb-4">{t('settings.theme')}</h2>
                <div className="grid grid-cols-3 gap-4">
                  {(['light', 'dark', 'system'] as const).map((themeOption) => (
                    <button
                      key={themeOption}
                      onClick={() => setTheme(themeOption)}
                      className={`p-4 rounded-lg border-2 transition-colors ${
                        theme === themeOption
                          ? 'border-[var(--color-primary)] bg-[var(--color-primary)]/10'
                          : 'border-[var(--border-base)] hover:border-[var(--text-muted)]'
                      }`}
                    >
                      <div className="flex items-center justify-between mb-2">
                        <span className="text-sm font-medium text-[var(--text-primary)]">
                          {themeOption === 'light' && t('settings.themeLight')}
                          {themeOption === 'dark' && t('settings.themeDark')}
                          {themeOption === 'system' && t('settings.themeSystem')}
                        </span>
                        {theme === themeOption && <CheckCircle className="w-4 h-4 text-[var(--color-primary)]" />}
                      </div>
                      <div
                        className={`h-12 rounded-lg ${
                          themeOption === 'light'
                            ? 'bg-white border border-gray-200'
                            : themeOption === 'dark'
                            ? 'bg-gray-900 border border-gray-700'
                            : 'bg-gradient-to-r from-white to-gray-900'
                        }`}
                      />
                    </button>
                  ))}
                </div>
              </div>
            )}
          </Card>
        </div>
      </div>
    </div>
  );
}