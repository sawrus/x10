import type { HTMLAttributes, PropsWithChildren } from 'react'

import { cn } from '../lib/cn'

type BadgeTone = 'danger' | 'muted' | 'success'

const toneClasses: Record<BadgeTone, string> = {
  danger: 'border-rose-400/30 bg-rose-400/10 text-rose-100',
  muted: 'border-white/10 bg-white/5 text-slate-200',
  success: 'border-emerald-400/30 bg-emerald-400/10 text-emerald-100',
}

export type BadgeProps = PropsWithChildren<
  HTMLAttributes<HTMLSpanElement> & {
    tone?: BadgeTone
  }
>

export function Badge({ children, className, tone = 'muted', ...props }: BadgeProps) {
  return (
    <span
      {...props}
      className={cn(
        'inline-flex items-center rounded-full border px-3 py-1 text-xs font-semibold uppercase tracking-[0.2em]',
        toneClasses[tone],
        className,
      )}
    >
      {children}
    </span>
  )
}
