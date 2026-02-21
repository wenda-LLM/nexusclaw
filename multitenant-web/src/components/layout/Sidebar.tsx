import { NavLink } from 'react-router-dom';
import {
  LayoutDashboard,
  Radio,
  FileText,
  BarChart,
  Wrench,
  Zap,
  Puzzle,
  Clock,
  Users,
  Settings,
  MessageSquare,
} from 'lucide-react';
import { useAuth } from '@/contexts/AuthContext';
import { t } from '@/lib/i18n';

const adminNavItems = [
  { to: '/', icon: LayoutDashboard, label: t('nav.dashboard') },
  { to: '/instances', icon: Radio, label: t('nav.instances') },
  { to: '/sessions', icon: FileText, label: t('nav.sessions') },
  { to: '/usage', icon: BarChart, label: t('nav.usage') },
  { to: '/tools', icon: Wrench, label: t('nav.tools') },
  { to: '/skills', icon: Zap, label: t('nav.skills') },
  { to: '/mcp', icon: Puzzle, label: t('nav.mcp') },
  { to: '/cron', icon: Clock, label: t('nav.cron') },
  { to: '/tenants', icon: Users, label: t('nav.tenants') },
  { to: '/config', icon: Settings, label: t('nav.config') },
];

const userNavItems = [
  { to: '/chat', icon: MessageSquare, label: t('nav.chat') },
  { to: '/settings', icon: Settings, label: t('nav.settings') },
];

export function Sidebar() {
  const { user } = useAuth();
  const isAdmin = user?.role === 'admin';
  const navItems = isAdmin ? adminNavItems : userNavItems;

  return (
    <aside className="fixed top-14 left-0 h-[calc(100vh-56px)] w-60 bg-[var(--bg-primary)] border-r border-[var(--border-base)] flex flex-col">
      <nav className="flex-1 overflow-y-auto py-4 px-3 space-y-1">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            end={item.to === '/'}
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? 'bg-[var(--color-primary)] text-white'
                  : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)]'
              }`
            }
          >
            <item.icon className="w-5 h-5 flex-shrink-0" />
            <span>{item.label}</span>
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}