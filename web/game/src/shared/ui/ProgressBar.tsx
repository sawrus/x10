import type { HTMLAttributes } from 'react'

import { cn } from '../lib/cn'

export type ProgressBarProps = HTMLAttributes<HTMLDivElement> & {
  label?: string
  max?: number
  value: number
}

export function ProgressBar({ className, label, max = 100, value, ...props }: ProgressBarProps) {
  const safeMax = Math.max(max, 1)
  const safeValue = Math.min(Math.max(value, 0), safeMax)
  const percent = Math.round((safeValue / safeMax) * 100)

  return (
    <div {...props} className={cn('flex w-full flex-col gap-2', className)}>
      <div className="flex items-center justify-between gap-3 text-sm text-slate-300">
        <span>{label ?? 'Progress'}</span>
        <span>{percent}%</span>
      </div>
      <div aria-hidden="true" className="h-3 rounded-full bg-slate-950/80">
        <div className="h-full rounded-full bg-cyan-300 transition-[width]" style={{ width: `${percent}%` }} />
      </div>
      <div aria-valuemax={safeMax} aria-valuemin={0} aria-valuenow={safeValue} className="sr-only" role="progressbar">
        {percent}%
      </div>
    </div>
  )
}
