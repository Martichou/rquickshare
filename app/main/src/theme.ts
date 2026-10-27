import { getCurrentWindow } from '@tauri-apps/api/window'

export type ThemePref = 'system' | 'light' | 'dark'

let pref: ThemePref = 'system'
let system: string | null = 'light'

function render() {
	const dark = pref === 'dark' || (pref === 'system' && system === 'dark')
	document.documentElement.classList.toggle('dark', dark)
}

export function applyThemePref(next: ThemePref) {
	pref = next
	render()
}

export async function initSystemTheme() {
	const win = getCurrentWindow()
	system = await win.theme().catch(() => null)
	render()
	win.onThemeChanged(({ payload }) => {
		system = payload
		render()
	}).catch(() => undefined)
}
