import { useState, useEffect, useRef, type FormEvent } from 'react';
import { Plus, Clock, Send, Bot, User, Loader2 } from 'lucide-react';
import { Button } from '@/components/common/Button';
import { Input } from '@/components/common/Input';
import { Modal } from '@/components/common/Modal';
import { Card } from '@/components/common/Card';
import { gateway } from '@/lib/gateway';
import { t } from '@/lib/i18n';
import type { ChatMessage, ChatSession, CronJob } from '@/types';

export function ChatPage() {
  const [sessions, setSessions] = useState<ChatSession[]>([]);
  const [currentSession, setCurrentSession] = useState<string>('main');
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [showCronModal, setShowCronModal] = useState(false);
  const [cronJobs, setCronJobs] = useState<CronJob[]>([]);
  const [cronForm, setCronForm] = useState({
    name: '',
    schedule: '0 8 * * *',
    message: '',
    sessionKey: 'main',
  });
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadSessions();
    loadCronJobs();
  }, []);

  useEffect(() => {
    if (currentSession) {
      loadHistory(currentSession);
    }
  }, [currentSession]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const loadSessions = async () => {
    try {
      const result = await gateway.request<{ sessions: ChatSession[] }>('sessions.list', {});
      setSessions(result.sessions || []);
    } catch (e) {
      console.error('Failed to load sessions:', e);
    }
  };

  const loadHistory = async (sessionKey: string) => {
    try {
      const result = await gateway.request<{ messages: ChatMessage[] }>('chat.history', { key: sessionKey, limit: 50 });
      setMessages(result.messages || []);
    } catch (e) {
      console.error('Failed to load history:', e);
    }
  };

  const loadCronJobs = async () => {
    try {
      const result = await gateway.request<{ jobs: CronJob[] }>('cron.list', {});
      setCronJobs(result.jobs || []);
    } catch (e) {
      console.error('Failed to load cron jobs:', e);
    }
  };

  const handleSend = async (e: FormEvent) => {
    e.preventDefault();
    if (!input.trim() || loading) return;

    const userMessage: ChatMessage = {
      id: `msg_${Date.now()}`,
      role: 'user',
      content: input.trim(),
      timestamp: new Date().toISOString(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInput('');
    setLoading(true);

    try {
      const result = await gateway.request<{ message: ChatMessage }>('chat.send', {
        message: input.trim(),
        session: currentSession,
      });
      
      if (result.message) {
        setMessages((prev) => [...prev, result.message]);
      }
    } catch (e) {
      console.error('Failed to send message:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateCron = async () => {
    try {
      await gateway.request('cron.add', {
        name: cronForm.name,
        schedule: cronForm.schedule,
        message: cronForm.message,
        sessionKey: cronForm.sessionKey,
      });
      setShowCronModal(false);
      setCronForm({ name: '', schedule: '0 8 * * *', message: '', sessionKey: 'main' });
      loadCronJobs();
    } catch (e) {
      console.error('Failed to create cron job:', e);
    }
  };

  const handleNewSession = () => {
    const newKey = `session_${Date.now()}`;
    setCurrentSession(newKey);
    setSessions((prev) => [{ key: newKey, updatedAt: new Date().toISOString() }, ...prev]);
  };

  return (
    <div className="flex h-[calc(100vh-88px)] gap-6">
      <div className="w-64 flex-shrink-0">
        <Card className="h-full flex flex-col">
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-semibold text-[var(--text-primary)]">会话</h3>
            <button
              onClick={handleNewSession}
              className="p-1.5 rounded-lg hover:bg-[var(--bg-hover)] text-[var(--text-secondary)]"
              title={t('chat.newSession')}
            >
              <Plus className="w-4 h-4" />
            </button>
          </div>
          <div className="flex-1 overflow-y-auto space-y-1">
            {sessions.map((session) => (
              <button
                key={session.key}
                onClick={() => setCurrentSession(session.key)}
                className={`w-full text-left px-3 py-2 rounded-lg text-sm truncate ${
                  currentSession === session.key
                    ? 'bg-[var(--color-primary)] text-white'
                    : 'hover:bg-[var(--bg-hover)] text-[var(--text-primary)]'
                }`}
              >
                {session.label || session.displayName || session.key}
              </button>
            ))}
          </div>
        </Card>
      </div>

      <div className="flex-1 flex flex-col min-w-0">
        <Card className="flex-1 flex flex-col p-0">
          <div className="flex items-center justify-between px-4 py-3 border-b border-[var(--border-base)]">
            <div className="flex items-center gap-2">
              <h3 className="font-semibold text-[var(--text-primary)]">
                {sessions.find((s) => s.key === currentSession)?.label || '新会话'}
              </h3>
            </div>
            <div className="flex items-center gap-1">
              <button
                onClick={() => setShowCronModal(true)}
                className="p-2 rounded-lg hover:bg-[var(--bg-hover)] text-[var(--text-secondary)]"
                title={t('chat.cronTask')}
              >
                <Clock className="w-4 h-4" />
              </button>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {messages.length === 0 ? (
              <div className="flex items-center justify-center h-full text-[var(--text-muted)]">
                <div className="text-center">
                  <Bot className="w-12 h-12 mx-auto mb-3 opacity-50" />
                  <p>开始一个新对话</p>
                </div>
              </div>
            ) : (
              messages.map((msg) => (
                <div
                  key={msg.id}
                  className={`flex gap-3 ${msg.role === 'user' ? 'flex-row-reverse' : ''}`}
                >
                  <div
                    className={`w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 ${
                      msg.role === 'user'
                        ? 'bg-[var(--color-primary)] text-white'
                        : 'bg-[var(--brand-500)] text-white'
                    }`}
                  >
                    {msg.role === 'user' ? <User className="w-4 h-4" /> : <Bot className="w-4 h-4" />}
                  </div>
                  <div
                    className={`max-w-[70%] rounded-lg px-4 py-2 ${
                      msg.role === 'user'
                        ? 'bg-[var(--color-primary)] text-white'
                        : 'bg-[var(--bg-secondary)] text-[var(--text-primary)]'
                    }`}
                  >
                    <p className="whitespace-pre-wrap">{msg.content}</p>
                    {msg.thinking && (
                      <details className="mt-2 text-sm opacity-75">
                        <summary className="cursor-pointer">思考过程</summary>
                        <pre className="mt-1 text-xs whitespace-pre-wrap">{msg.thinking}</pre>
                      </details>
                    )}
                  </div>
                </div>
              ))
            )}
            {loading && (
              <div className="flex gap-3">
                <div className="w-8 h-8 rounded-full bg-[var(--brand-500)] text-white flex items-center justify-center">
                  <Bot className="w-4 h-4" />
                </div>
                <div className="bg-[var(--bg-secondary)] rounded-lg px-4 py-2">
                  <Loader2 className="w-4 h-4 animate-spin text-[var(--text-secondary)]" />
                </div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>

          <form onSubmit={handleSend} className="px-4 py-3 border-t border-[var(--border-base)]">
            <div className="flex gap-2">
              <Input
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder={t('chat.placeholder')}
                className="flex-1"
                disabled={loading}
              />
              <Button type="submit" disabled={loading || !input.trim()}>
                <Send className="w-4 h-4" />
              </Button>
            </div>
          </form>
        </Card>
      </div>

      <Modal
        isOpen={showCronModal}
        onClose={() => setShowCronModal(false)}
        title={t('cron.add')}
        size="md"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCronModal(false)}>
              {t('common.cancel')}
            </Button>
            <Button onClick={handleCreateCron} disabled={!cronForm.name || !cronForm.message}>
              {t('common.add')}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t('cron.name')}
            value={cronForm.name}
            onChange={(e) => setCronForm({ ...cronForm, name: e.target.value })}
            placeholder="每日早安提醒"
          />
          <Input
            label={t('cron.schedule')}
            value={cronForm.schedule}
            onChange={(e) => setCronForm({ ...cronForm, schedule: e.target.value })}
            placeholder="0 8 * * *"
          />
          <Input
            label={t('cron.message')}
            value={cronForm.message}
            onChange={(e) => setCronForm({ ...cronForm, message: e.target.value })}
            placeholder="每天早上8点向我问好"
          />
          <Input
            label={t('cron.session')}
            value={cronForm.sessionKey}
            onChange={(e) => setCronForm({ ...cronForm, sessionKey: e.target.value })}
            placeholder="main"
          />
          
          {cronJobs.length > 0 && (
            <div className="mt-4">
              <h4 className="text-sm font-medium text-[var(--text-primary)] mb-2">现有定时任务</h4>
              <div className="space-y-2">
                {cronJobs.map((job) => (
                  <div
                    key={job.id}
                    className="flex items-center justify-between px-3 py-2 bg-[var(--bg-secondary)] rounded-lg text-sm"
                  >
                    <span className="text-[var(--text-primary)]">{job.name}</span>
                    <span className="text-[var(--text-secondary)]">{job.schedule}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </Modal>
    </div>
  );
}