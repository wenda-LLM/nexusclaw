import { Sun, Moon, Monitor, LogOut, ChevronDown } from 'lucide-react';
import { useState } from 'react';
import { useAuth } from '@/contexts/AuthContext';
import { useTheme } from '@/contexts/ThemeContext';
import { t } from '@/lib/i18n';

export function Header() {
  const { user, logout } = useAuth();
  const { theme, setTheme } = useTheme();
  const [showUserMenu, setShowUserMenu] = useState(false);
  const [showThemeMenu, setShowThemeMenu] = useState(false);

  const handleThemeChange = (newTheme: 'light' | 'dark' | 'system') => {
    setTheme(newTheme);
    setShowThemeMenu(false);
  };

  const handleLogout = () => {
    logout();
  };

  const ThemeIcon = theme === 'dark' ? Moon : theme === 'light' ? Sun : Monitor;

  return (
    <header className="fixed top-0 left-0 right-0 h-14 bg-[var(--bg-primary)] border-b border-[var(--border-base)] flex items-center justify-between px-4 z-50">
      <div className="flex items-center gap-3">
        <div className="h-8 w-8 rounded-lg bg-[var(--brand-500)] flex items-center justify-center text-white font-bold text-sm">
          ZC
        </div>
        <span className="text-lg font-semibold text-[var(--text-primary)]">
          {t('app.name')}
        </span>
        <span className="text-sm text-[var(--text-secondary)]">
          {t('app.subtitle')}
        </span>
      </div>

      <div className="flex items-center gap-2">
        <div className="relative">
          <button
            onClick={() => setShowThemeMenu(!showThemeMenu)}
            className="p-2 rounded-lg hover:bg-[var(--bg-hover)] transition-colors"
            title={t('settings.theme')}
          >
            <ThemeIcon className="w-5 h-5 text-[var(--text-secondary)]" />
          </button>
          
          {showThemeMenu && (
            <div className="absolute right-0 top-full mt-1 w-36 bg-[var(--bg-primary)] border border-[var(--border-base)] rounded-lg shadow-lg py-1">
              <button
                onClick={() => handleThemeChange('light')}
                className={`w-full px-3 py-2 text-left flex items-center gap-2 hover:bg-[var(--bg-hover)] ${
                  theme === 'light' ? 'text-[var(--color-primary)]' : 'text-[var(--text-primary)]'
                }`}
              >
                <Sun className="w-4 h-4" />
                {t('settings.themeLight')}
              </button>
              <button
                onClick={() => handleThemeChange('dark')}
                className={`w-full px-3 py-2 text-left flex items-center gap-2 hover:bg-[var(--bg-hover)] ${
                  theme === 'dark' ? 'text-[var(--color-primary)]' : 'text-[var(--text-primary)]'
                }`}
              >
                <Moon className="w-4 h-4" />
                {t('settings.themeDark')}
              </button>
              <button
                onClick={() => handleThemeChange('system')}
                className={`w-full px-3 py-2 text-left flex items-center gap-2 hover:bg-[var(--bg-hover)] ${
                  theme === 'system' ? 'text-[var(--color-primary)]' : 'text-[var(--text-primary)]'
                }`}
              >
                <Monitor className="w-4 h-4" />
                {t('settings.themeSystem')}
              </button>
            </div>
          )}
        </div>

        <div className="relative">
          <button
            onClick={() => setShowUserMenu(!showUserMenu)}
            className="flex items-center gap-2 px-3 py-1.5 rounded-lg hover:bg-[var(--bg-hover)] transition-colors"
          >
            <div className="w-8 h-8 rounded-full bg-[var(--brand-500)] flex items-center justify-center text-white">
              {user?.name?.charAt(0).toUpperCase() || 'U'}
            </div>
            <span className="text-sm text-[var(--text-primary)]">{user?.name}</span>
            <ChevronDown className="w-4 h-4 text-[var(--text-secondary)]" />
          </button>

          {showUserMenu && (
            <div className="absolute right-0 top-full mt-1 w-48 bg-[var(--bg-primary)] border border-[var(--border-base)] rounded-lg shadow-lg py-1">
              <div className="px-3 py-2 border-b border-[var(--border-base)]">
                <p className="text-sm font-medium text-[var(--text-primary)]">{user?.name}</p>
                <p className="text-xs text-[var(--text-secondary)]">{user?.email}</p>
              </div>
              <button
                onClick={handleLogout}
                className="w-full px-3 py-2 text-left flex items-center gap-2 text-[var(--color-danger)] hover:bg-[var(--bg-hover)]"
              >
                <LogOut className="w-4 h-4" />
                {t('nav.logout')}
              </button>
            </div>
          )}
        </div>
      </div>
    </header>
  );
}