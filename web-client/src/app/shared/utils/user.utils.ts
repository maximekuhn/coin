export function formatUsername(username: string, userId: string): string {
  const suffix = userId.slice(-7);
  return `${username}#${suffix}`;
}
