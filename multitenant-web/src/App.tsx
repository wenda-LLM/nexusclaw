import { Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from '@/contexts/AuthContext';
import { ThemeProvider } from '@/contexts/ThemeContext';
import { Layout } from '@/components/layout/Layout';
import { LoginPage } from '@/pages/Login';
import { RegisterPage } from '@/pages/Register';
import { ChatPage } from '@/pages/Chat';
import { SettingsPage } from '@/pages/Settings';
import { OverviewPage } from '@/pages/Overview';
import { InstancesPage } from '@/pages/Instances';
import { SessionsPage } from '@/pages/Sessions';
import { UsagePage } from '@/pages/Usage';
import { ToolsPage } from '@/pages/Tools';
import { SkillsPage } from '@/pages/Skills';
import { McpPage } from '@/pages/Mcp';
import { CronPage } from '@/pages/Cron';
import { TenantsPage } from '@/pages/Tenants';
import { ConfigPage } from '@/pages/Config';

function AppRoutes() {
  const { isAuthenticated, isLoading, user } = useAuth();

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[var(--bg-base)]">
        <div className="animate-spin w-8 h-8 border-2 border-[var(--color-primary)] border-t-transparent rounded-full" />
      </div>
    );
  }

  if (!isAuthenticated) {
    return (
      <Routes>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/register" element={<RegisterPage />} />
        <Route path="*" element={<Navigate to="/login" replace />} />
      </Routes>
    );
  }

  const isAdmin = user?.role === 'admin';

  return (
    <Routes>
      <Route element={<Layout />}>
        {isAdmin ? (
          <>
            <Route path="/" element={<OverviewPage />} />
            <Route path="/instances" element={<InstancesPage />} />
            <Route path="/sessions" element={<SessionsPage />} />
            <Route path="/usage" element={<UsagePage />} />
            <Route path="/tools" element={<ToolsPage />} />
            <Route path="/skills" element={<SkillsPage />} />
            <Route path="/mcp" element={<McpPage />} />
            <Route path="/cron" element={<CronPage />} />
            <Route path="/tenants" element={<TenantsPage />} />
            <Route path="/config" element={<ConfigPage />} />
          </>
        ) : null}
        <Route path="/chat" element={<ChatPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
      <Route path="*" element={<Navigate to={isAdmin ? '/' : '/chat'} replace />} />
    </Routes>
  );
}

export default function App() {
  return (
    <ThemeProvider>
      <AuthProvider>
        <AppRoutes />
      </AuthProvider>
    </ThemeProvider>
  );
}