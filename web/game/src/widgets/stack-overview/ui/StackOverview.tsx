const stackItems = ['Vite', 'React', 'TypeScript', 'TanStack Query', 'Zustand', 'React Router', 'Tailwind CSS']

export function StackOverview() {
  return (
    <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
      {stackItems.map((item) => (
        <div key={item} className="rounded-2xl border border-white/10 bg-slate-900/70 px-4 py-3 text-sm font-semibold">
          {item}
        </div>
      ))}
    </div>
  )
}
