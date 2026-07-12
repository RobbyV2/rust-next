declare module 'bun:test' {
  interface Matchers<T> {
    toBe(expected: T): void
    toEqual(expected: T): void
    toBeNull(): void
    toBeUndefined(): void
    toHaveLength(length: number): void
  }
  export function expect<T>(actual: T): Matchers<T>
  export function test(name: string, fn: () => void | Promise<void>): void
  export function describe(name: string, fn: () => void): void
  export function beforeEach(fn: () => void | Promise<void>): void
}
