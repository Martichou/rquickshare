export * from './types';

export function opt<T>(v?: T) {
	return v ?? null;
}