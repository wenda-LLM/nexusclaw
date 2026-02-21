import { useState, useEffect } from 'react';
import { RefreshCw, Save, Settings, Code } from 'lucide-react';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { ConfigSnapshot } from '@/types';

export function ConfigPage() {
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [config, setConfig] = useState<ConfigSnapshot | null>(null);
  const [rawConfig, setRawConfig] = useState('');
  const [mode, setMode] = useState<'form' | 'raw'>('form');
  const [message, setMessage] = useState('');

  const loadConfig = async () => {
    setLoading(true);
    try {
      const result = await gateway.request<ConfigSnapshot>('config.get', {});
      setConfig(result);
      setRawConfig(JSON.stringify(result.config, null, 2));
    } catch (e) {
      console.error('Failed to load config:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    setMessage('');
    try {
      let newConfig: Record<string, unknown>;
      
      if (mode === 'raw') {
        newConfig = JSON.parse(rawConfig);
      } else {
        newConfig = config?.config || {};
      }

      await gateway.request('config.set', { config: newConfig });
      setMessage('配置已保存');
      loadConfig();
    } catch (e) {
      setMessage(e instanceof Error ? e.message : '保存失败');
    } finally {
      setSaving(false);
    }
  };

  const handleApply = async () => {
    setSaving(true);
    try {
      await gateway.request('config.apply', {});
      setMessage('配置已应用');
    } catch (e) {
      setMessage(e instanceof Error ? e.message : '应用失败');
    } finally {
      setSaving(false);
    }
  };

  useEffect(() => {
    loadConfig();
  }, []);

  const renderFormConfig = (obj: Record<string, unknown>, prefix = '') => {
    return Object.entries(obj).map(([key, value]) => {
      const fullKey = prefix ? `${prefix}.${key}` : key;
      
      if (value && typeof value === 'object' && !Array.isArray(value)) {
        return (
          <div key={fullKey} className="mb-4">
            <h4 className="text-sm font-medium text-[var(--text-secondary)] uppercase mb-2">{key}</h4>
            <div className="pl-4 border-l border-[var(--border-base)]">
              {renderFormConfig(value as Record<string, unknown>, fullKey)}
            </div>
          </div>
        );
      }

      return (
        <div key={fullKey} className="mb-3">
          <label className="block text-sm text-[var(--text-primary)] mb-1">{key}</label>
          {typeof value === 'boolean' ? (
            <input
              type="checkbox"
              checked={value}
              onChange={(e) => {
                if (!config) return;
                const newConfig = { ...config.config };
                const keys = fullKey.split('.');
                let current = newConfig;
                for (let i = 0; i < keys.length - 1; i++) {
                  current = current[keys[i]] as Record<string, unknown>;
                }
                current[keys[keys.length - 1]] = e.target.checked;
                setConfig({ ...config, config: newConfig });
              }}
              className="w-4 h-4"
            />
          ) : typeof value === 'number' ? (
            <input
              type="number"
              value={value}
              onChange={(e) => {
                if (!config) return;
                const newConfig = { ...config.config };
                const keys = fullKey.split('.');
                let current = newConfig;
                for (let i = 0; i < keys.length - 1; i++) {
                  current = current[keys[i]] as Record<string, unknown>;
                }
                current[keys[keys.length - 1]] = Number(e.target.value);
                setConfig({ ...config, config: newConfig });
              }}
              className="w-full px-3 py-1.5 bg-[var(--bg-secondary)] border border-[var(--border-base)] rounded text-[var(--text-primary)]"
            />
          ) : (
            <input
              type="text"
              value={String(value || '')}
              onChange={(e) => {
                if (!config) return;
                const newConfig = { ...config.config };
                const keys = fullKey.split('.');
                let current = newConfig;
                for (let i = 0; i < keys.length - 1; i++) {
                  current = current[keys[i]] as Record<string, unknown>;
                }
                current[keys[keys.length - 1]] = e.target.value;
                setConfig({ ...config, config: newConfig });
              }}
              className="w-full px-3 py-1.5 bg-[var(--bg-secondary)] border border-[var(--border-base)] rounded text-[var(--text-primary)]"
            />
          )}
        </div>
      );
    });
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-[var(--text-primary)]">{t('config.title')}</h1>
        <div className="flex items-center gap-2">
          <Button variant="secondary" onClick={loadConfig} loading={loading}>
            <RefreshCw className="w-4 h-4 mr-2" />
            刷新
          </Button>
        </div>
      </div>

      <div className="flex items-center gap-4 mb-4">
        <div className="flex bg-[var(--bg-secondary)] rounded-lg p-1">
          <button
            onClick={() => setMode('form')}
            className={`px-4 py-2 rounded-md text-sm transition-colors ${
              mode === 'form' ? 'bg-[var(--color-primary)] text-white' : 'text-[var(--text-secondary)]'
            }`}
          >
            <Settings className="w-4 h-4 inline mr-2" />
            表单
          </button>
          <button
            onClick={() => setMode('raw')}
            className={`px-4 py-2 rounded-md text-sm transition-colors ${
              mode === 'raw' ? 'bg-[var(--color-primary)] text-white' : 'text-[var(--text-secondary)]'
            }`}
          >
            <Code className="w-4 h-4 inline mr-2" />
            原始
          </button>
        </div>

        <div className="flex-1" />

        {message && (
          <span className={`text-sm ${message.includes('已') ? 'text-[var(--color-success)]' : 'text-[var(--color-danger)]'}`}>
            {message}
          </span>
        )}

        <Button onClick={handleSave} loading={saving}>
          <Save className="w-4 h-4 mr-2" />
          {t('config.save')}
        </Button>
        <Button variant="secondary" onClick={handleApply} loading={saving}>
          {t('config.apply')}
        </Button>
      </div>

      <Card>
        {loading ? (
          <div className="text-center py-12 text-[var(--text-muted)]">加载中...</div>
        ) : mode === 'form' ? (
          <div className="max-h-[60vh] overflow-y-auto">
            {config?.config && renderFormConfig(config.config)}
          </div>
        ) : (
          <textarea
            value={rawConfig}
            onChange={(e) => setRawConfig(e.target.value)}
            className="w-full h-[60vh] bg-[var(--bg-secondary)] border border-[var(--border-base)] rounded-lg p-4 font-mono text-sm text-[var(--text-primary)] resize-none"
            spellCheck={false}
          />
        )}
      </Card>
    </div>
  );
}