export function getBasePath(): string {
  if (typeof window === 'undefined') {
    return ''
  }

  const nextData = (window as unknown as Record<string, unknown>).__NEXT_DATA__ as
    { assetPrefix?: string } | undefined
  if (nextData?.assetPrefix) {
    return nextData.assetPrefix.trim()
  }

  return ''
}
