// Fakes the Tauri backend so the UI runs in a plain browser (vite dev only).
// URL options: ?theme=light|dark and ?demo=1 (adds sample devices and a request).
import { mockIPC, mockWindows } from '@tauri-apps/api/mocks'

const params = new URLSearchParams(window.location.search)
const theme = params.get('theme') ?? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
const settings = new Map<string, unknown>()
const listeners = new Map<string, Array<{ id: number, handler: number }>>()
let nextEventId = 1

// tauri 2.0.3 has no event mocking, so deliver events through the registered callbacks.
function emit(event: string, payload: unknown) {
	for (const l of listeners.get(event) ?? []) {
		const cb = (window as unknown as Record<string, ((e: unknown) => void) | undefined>)[`_${l.handler}`]
		cb?.({ event, id: l.id, payload })
	}
}

export function installDevMock() {
	mockWindows('main')
	mockIPC((async (cmd: string, args?: unknown) => {
		const a = (args ?? {}) as { key?: string, value?: unknown, event?: string, handler?: number }
		switch (cmd) {
			case 'get_hostname': return 'HPpavilionpro'
			case 'plugin:app|version': return '0.11.5'
			case 'plugin:event|listen': {
				const id = nextEventId++
				listeners.set(a.event ?? '', [...(listeners.get(a.event ?? '') ?? []), { id, handler: a.handler ?? 0 }])
				return id
			}
			case 'plugin:window|theme': return theme
			case 'plugin:store|get_store':
			case 'plugin:store|load': return 1
			case 'plugin:store|has': return settings.has(a.key ?? '')
			case 'plugin:store|get': return [settings.get(a.key ?? '') ?? null, settings.has(a.key ?? '')]
			case 'plugin:store|set': settings.set(a.key ?? '', a.value); return null
			case 'plugin:notification|is_permission_granted': return true
			case 'plugin:autostart|is_enabled': return false
			default: return null
		}
	}) as Parameters<typeof mockIPC>[0])

	if (params.get('demo')) {
		setTimeout(() => {
			emit('rs2js_endpointinfo', { fullname: 'a', id: 'a', name: 'Pixel 8', ip: '192.168.1.20', port: '1', rtype: 'Phone', present: true })
			emit('rs2js_endpointinfo', { fullname: 'b', id: 'b', name: 'Work laptop', ip: '192.168.1.21', port: '1', rtype: 'Laptop', present: true })
			emit('rs2js_channelmessage', {
				id: 'c', direction: 'LibToHost', action: null, rtype: 'Inbound', state: 'WaitingForUserConsent',
				meta: { source: { name: 'Galaxy S24', device_type: 'Phone' }, files: ['holiday.jpg', 'notes.pdf'], pin_code: '4821', text_description: null, total_bytes: 1024, ack_bytes: 0, destination: null, payload_kind: null, text_payload: null },
			})
		}, 3000)
	}
}
