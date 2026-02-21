import { useState, useEffect } from 'react';
import { RefreshCw, Plus, Users, Trash2, UserPlus } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { Modal } from '@/components/common/Modal';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { Tenant, TenantUser } from '@/types';

export function TenantsPage() {
  const [loading, setLoading] = useState(false);
  const [tenants, setTenants] = useState<Tenant[]>([]);
  const [selectedTenant, setSelectedTenant] = useState<Tenant | null>(null);
  const [users, setUsers] = useState<TenantUser[]>([]);
  const [showTenantModal, setShowTenantModal] = useState(false);
  const [showUserModal, setShowUserModal] = useState(false);
  const [tenantForm, setTenantForm] = useState({ name: '' });
  const [userForm, setUserForm] = useState<{ email: string; name: string; role: 'admin' | 'member' }>({ email: '', name: '', role: 'member' });

  const loadTenants = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<{ tenants: Tenant[] }>('tenants.list', {});
      setTenants(result.tenants || []);
    } catch (e) {
      console.error('Failed to load tenants:', e);
    } finally {
      setLoading(false);
    }
  };

  const loadUsers = async (tenantId: string) => {
    try {
      const result = await gateway.request<{ users: TenantUser[] }>('tenants.users', { tenantId });
      setUsers(result.users || []);
    } catch (e) {
      console.error('Failed to load users:', e);
    }
  };

  const handleCreateTenant = async () => {
    try {
      await gateway.request('tenants.create', tenantForm);
      setShowTenantModal(false);
      setTenantForm({ name: '' });
      loadTenants();
    } catch (e) {
      console.error('Failed to create tenant:', e);
    }
  };

  const handleDeleteTenant = async (id: string) => {
    if (!confirm('确定删除此租户？')) return;
    try {
      await gateway.request('tenants.delete', { id });
      loadTenants();
    } catch (e) {
      console.error('Failed to delete tenant:', e);
    }
  };

  const handleAddUser = async () => {
    if (!selectedTenant) return;
    try {
      await gateway.request('tenants.addUser', { tenantId: selectedTenant.id, ...userForm });
      setShowUserModal(false);
      setUserForm({ email: '', name: '', role: 'member' });
      loadUsers(selectedTenant.id);
    } catch (e) {
      console.error('Failed to add user:', e);
    }
  };

  const handleSelectTenant = (tenant: Tenant) => {
    setSelectedTenant(tenant);
    loadUsers(tenant.id);
  };

  useEffect(() => {
    loadTenants();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('tenants.title')}</h1>
        <Button variant="secondary" onClick={loadTenants} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card
          title={t('tenants.title')}
          action={
            <Button size="sm" onClick={() => setShowTenantModal(true)}>
              <Plus className="w-4 h-4 mr-1" />
              {t('tenants.add')}
            </Button>
          }
        >
          {tenants.length === 0 ? (
            <div className="text-center py-8 text-[var(--text-muted)]">
              <Users className="w-10 h-10 mx-auto mb-3 opacity-50" />
              <p>{t('common.noData')}</p>
            </div>
          ) : (
            <div className="space-y-2">
              {tenants.map((tenant) => (
                <div
                  key={tenant.id}
                  onClick={() => handleSelectTenant(tenant)}
                  className={`flex items-center justify-between p-3 rounded-lg cursor-pointer transition-colors ${
                    selectedTenant?.id === tenant.id
                      ? 'bg-[var(--color-primary)]/10 border border-[var(--color-primary)]'
                      : 'bg-[var(--bg-secondary)] hover:bg-[var(--bg-hover)]'
                  }`}
                >
                  <div>
                    <p className="font-medium text-[var(--text-primary)]">{tenant.name}</p>
                    <p className="text-sm text-[var(--text-secondary)]">
                      {tenant.userCount} 用户 • {new Date(tenant.createdAt).toLocaleDateString()}
                    </p>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleDeleteTenant(tenant.id);
                    }}
                    className="text-[var(--color-danger)]"
                  >
                    <Trash2 className="w-4 h-4" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </Card>

        <Card
          title={selectedTenant ? `${selectedTenant.name} - 用户` : t('tenants.users.title')}
          action={
            selectedTenant ? (
              <Button size="sm" onClick={() => setShowUserModal(true)}>
                <UserPlus className="w-4 h-4 mr-1" />
                {t('tenants.users.add')}
              </Button>
            ) : null
          }
        >
          {!selectedTenant ? (
            <div className="text-center py-8 text-[var(--text-muted)]">
              <p>选择租户查看用户</p>
            </div>
          ) : users.length === 0 ? (
            <div className="text-center py-8 text-[var(--text-muted)]">
              <p>{t('common.noData')}</p>
            </div>
          ) : (
            <div className="space-y-2">
              {users.map((user) => (
                <div key={user.id} className="flex items-center justify-between p-3 bg-[var(--bg-secondary)] rounded-lg">
                  <div>
                    <p className="font-medium text-[var(--text-primary)]">{user.name}</p>
                    <p className="text-sm text-[var(--text-secondary)]">{user.email}</p>
                  </div>
                  <span className={`text-xs px-2 py-1 rounded ${
                    user.role === 'admin' ? 'bg-[var(--color-primary)]/10 text-[var(--color-primary)]' : 'bg-[var(--bg-tertiary)]'
                  }`}>
                    {user.role === 'admin' ? t('tenants.users.admin') : t('tenants.users.member')}
                  </span>
                </div>
              ))}
            </div>
          )}
        </Card>
      </div>

      <Modal
        isOpen={showTenantModal}
        onClose={() => setShowTenantModal(false)}
        title={t('tenants.add')}
        size="sm"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowTenantModal(false)}>
              {t('common.cancel')}
            </Button>
            <Button onClick={handleCreateTenant} disabled={!tenantForm.name}>
              {t('common.add')}
            </Button>
          </>
        }
      >
        <Input
          label={t('tenants.name')}
          value={tenantForm.name}
          onChange={(e) => setTenantForm({ name: e.target.value })}
          placeholder="公司名称"
        />
      </Modal>

      <Modal
        isOpen={showUserModal}
        onClose={() => setShowUserModal(false)}
        title={t('tenants.users.add')}
        size="sm"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowUserModal(false)}>
              {t('common.cancel')}
            </Button>
            <Button onClick={handleAddUser} disabled={!userForm.email || !userForm.name}>
              {t('common.add')}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t('tenants.users.email')}
            type="email"
            value={userForm.email}
            onChange={(e) => setUserForm({ ...userForm, email: e.target.value })}
            placeholder="user@example.com"
          />
          <Input
            label="昵称"
            value={userForm.name}
            onChange={(e) => setUserForm({ ...userForm, name: e.target.value })}
            placeholder="用户昵称"
          />
          <div>
            <label className="block text-sm font-medium text-[var(--text-primary)] mb-1.5">
              {t('tenants.users.role')}
            </label>
            <select
              value={userForm.role}
              onChange={(e) => setUserForm({ ...userForm, role: e.target.value as 'admin' | 'member' })}
              className="w-full px-4 py-2.5 bg-[var(--bg-secondary)] border border-[var(--border-base)] rounded-lg text-[var(--text-primary)]"
            >
              <option value="member">{t('tenants.users.member')}</option>
              <option value="admin">{t('tenants.users.admin')}</option>
            </select>
          </div>
        </div>
      </Modal>
    </div>
  );
}