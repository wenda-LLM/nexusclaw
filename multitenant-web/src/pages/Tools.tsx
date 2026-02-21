import { useState, useEffect } from 'react';
import { RefreshCw, Wrench, ToggleLeft, ToggleRight } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { Tool } from '@/types';

export function ToolsPage() {
  const [loading, setLoading] = useState(false);
  const [tools, setTools] = useState<Tool[]>([]);
  const [search, setSearch] = useState('');

  const loadTools = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<{ tools: Tool[] }>('tools.list', {});
      setTools(result.tools || []);
    } catch (e) {
      console.error('Failed to load tools:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleToggle = async (name: string, enabled: boolean) => {
    try {
      await gateway.request('tools.toggle', { name, enabled: !enabled });
      loadTools();
    } catch (e) {
      console.error('Failed to toggle tool:', e);
    }
  };

  useEffect(() => {
    loadTools();
  }, []);

  const filteredTools = tools.filter(
    (tool) =>
      tool.name.toLowerCase().includes(search.toLowerCase()) ||
      tool.description.toLowerCase().includes(search.toLowerCase())
  );

  const groupedTools = filteredTools.reduce((acc, tool) => {
    const category = tool.category || 'other';
    if (!acc[category]) acc[category] = [];
    acc[category].push(tool);
    return acc;
  }, {} as Record<string, Tool[]>);

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('tools.title')}</h1>
        <Button variant="secondary" onClick={loadTools} loading={loading}>
          <RefreshCw className="w-4 h-4 mr-2" />
          刷新
        </Button>
      </div>

      <div className="mb-4">
        <Input
          placeholder={t('common.search')}
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="max-w-xs"
        />
      </div>

      {Object.entries(groupedTools).map(([category, categoryTools]) => (
        <Card key={category} title={category} className="mb-4">
          <div className="space-y-3">
            {categoryTools.map((tool) => (
              <div
                key={tool.name}
                className="flex items-center justify-between py-2 border-b border-[var(--border-base)] last:border-0"
              >
                <div className="flex items-center gap-3">
                  <Wrench className="w-5 h-5 text-[var(--text-muted)]" />
                  <div>
                    <p className="text-[var(--text-primary)] font-medium">{tool.name}</p>
                    <p className="text-sm text-[var(--text-secondary)]">{tool.description}</p>
                  </div>
                </div>
                <button
                  onClick={() => handleToggle(tool.name, tool.enabled)}
                  className="text-[var(--text-secondary)] hover:text-[var(--color-primary)]"
                >
                  {tool.enabled ? (
                    <ToggleRight className="w-6 h-6 text-[var(--color-success)]" />
                  ) : (
                    <ToggleLeft className="w-6 h-6" />
                  )}
                </button>
              </div>
            ))}
          </div>
        </Card>
      ))}

      {filteredTools.length === 0 && (
        <div className="text-center py-12 text-[var(--text-muted)]">
          <p>{t('common.noData')}</p>
        </div>
      )}
    </div>
  );
}