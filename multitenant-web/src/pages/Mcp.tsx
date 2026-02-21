import { useState, useEffect } from 'react';
import { RefreshCw, Plus, Puzzle, Edit, Trash2, Circle } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { Modal } from '@/components/common/Modal';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { McpServer } from '@/types';

export function McpPage() {
  const [loading, setLoading] = useState(false);
  const [servers, setServers] = useState<McpServer[]>([]);
  const [showModal, setShowModal] = useState(false);
  const [editingServer, setEditingServer] = useState<McpServer | null>(null);
  const [form, setForm] = useState({ name: '', url: '', description: '' });

  const loadServers = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<{ servers: McpServer[] }>('mcp.list', {});
      setServers(result.servers || []);
    } catch (e) {
      console.error('Failed to load MCP servers:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      if (editingServer) {
        await gateway.request('mcp.update', { id: editingServer.id, ...form });
      } else {
        await gateway.request('mcp.add', form);
      }
      setShowModal(false);
      setEditingServer(null);
      setForm({ name: '', url: '', description: '' });
      loadServers();
    } catch (e) {
      console.error('Failed to save MCP server:', e);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('确定删除此 MCP 服务器？')) return;
    try {
      await gateway.request('mcp.remove', { id });
      loadServers();
    } catch (e) {
      console.error('Failed to delete MCP server:', e);
    }
  };

  useEffect(() => {
    loadServers();
  }, []);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('mcp.title')}</h1>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={loadServers} loading={loading}>
            <RefreshCw className="w-4 h-4 mr-2" />
            刷新
          </Button>
          <Button onClick={() => setShowModal(true)}>
            <Plus className="w-4 h-4 mr-2" />
            {t('mcp.add')}
          </Button>
        </div>
      </div>

      <Card>
        {servers.length === 0 ? (
          <div className="text-center py-12 text-[var(--text-muted)]">
            <Puzzle className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>{t('common.noData')}</p>
          </div>
        ) : (
          <div className="space-y-4">
            {servers.map((server) => (
              <div
                key={server.id}
                className="flex items-center justify-between py-3 border-b border-[var(--border-base)] last:border-0"
              >
                <div className="flex items-center gap-3">
                  <Circle
                    className={`w-3 h-3 ${server.status === 'connected' ? 'text-[var(--color-success)]' : 'text-[var(--color-danger)]'}`}
                    fill="currentColor"
                  />
                  <div>
                    <p className="font-medium text-[var(--text-primary)]">{server.name}</p>
                    <p className="text-sm text-[var(--text-secondary)]">{server.url}</p>
                    {server.description && (
                      <p className="text-xs text-[var(--text-muted)]">{server.description}</p>
                    )}
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <span className={`text-xs px-2 py-1 rounded ${
                    server.status === 'connected' ? 'bg-[var(--color-success)]/10 text-[var(--color-success)]' : 'bg-[var(--color-danger)]/10 text-[var(--color-danger)]'
                  }`}>
                    {server.status === 'connected' ? t('mcp.connected') : t('mcp.disconnected')}
                  </span>
                  <Button variant="ghost" size="sm" onClick={() => {
                    setEditingServer(server);
                    setForm({ name: server.name, url: server.url, description: server.description || '' });
                    setShowModal(true);
                  }}>
                    <Edit className="w-4 h-4" />
                  </Button>
                  <Button variant="ghost" size="sm" onClick={() => handleDelete(server.id)} className="text-[var(--color-danger)]">
                    <Trash2 className="w-4 h-4" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      <Modal
        isOpen={showModal}
        onClose={() => {
          setShowModal(false);
          setEditingServer(null);
          setForm({ name: '', url: '', description: '' });
        }}
        title={editingServer ? t('mcp.edit') : t('mcp.add')}
        size="md"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowModal(false)}>
              {t('common.cancel')}
            </Button>
            <Button onClick={handleSave} disabled={!form.name || !form.url}>
              {t('common.save')}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t('mcp.name')}
            value={form.name}
            onChange={(e) => setForm({ ...form, name: e.target.value })}
            placeholder="Filesystem"
          />
          <Input
            label={t('mcp.url')}
            value={form.url}
            onChange={(e) => setForm({ ...form, url: e.target.value })}
            placeholder="sse://localhost:3000/filesystem"
          />
          <Input
            label={t('mcp.description')}
            value={form.description}
            onChange={(e) => setForm({ ...form, description: e.target.value })}
            placeholder="本地文件系统访问"
          />
        </div>
      </Modal>
    </div>
  );
}