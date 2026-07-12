import { describe, expect, test } from 'bun:test'
import { resolveApiBase } from '../api'

describe('resolveApiBase', () => {
  test('NEXT_PUBLIC_API_URL set, trailing slashes stripped', () => {
    expect(resolveApiBase('https://x.com//', false)).toBe('https://x.com/api')
  })

  test('url wins over window', () => {
    expect(resolveApiBase('https://x.com', true)).toBe('https://x.com/api')
  })

  test('unset with window -> relative', () => {
    expect(resolveApiBase(undefined, true)).toBe('/api')
  })

  test('unset without window -> host/port', () => {
    expect(resolveApiBase(undefined, false, 'srv', '9000')).toBe('http://srv:9000/api')
  })

  test('unset without window, no host/port -> defaults', () => {
    expect(resolveApiBase(undefined, false)).toBe('http://127.0.0.1:3000/api')
    expect(resolveApiBase(undefined, false, '', '')).toBe('http://127.0.0.1:3000/api')
  })
})
