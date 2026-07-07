import { forwardRef, useId } from 'react'
import type { SelectHTMLAttributes } from 'react'

import { cn } from '../lib/cn'

export type SelectOption = {
  label: string
  value: string
}

export type SelectProps = SelectHTMLAttributes<HTMLSelectElement> & {
  error?: string
  hint?: string
  label?: string
  options: SelectOption[]
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(function Select(
  { className, error, hint, id, label, options, ...props },
  ref,
) {
  const generatedId = useId()
  const selectId = id ?? generatedId
  const messageId = `${selectId}-message`

  return (
    <label className="flex w-full flex-col gap-2" htmlFor={selectId}>
      {label ? <span className="text-sm font-semibold text-slate-200">{label}</span> : null}
      <select
        {...props}
        ref={ref}
        id={selectId}
        aria-describedby={error || hint ? messageId : undefined}
        aria-invalid={error ? 'true' : undefined}
        className={cn(
          'w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-sm text-slate-100 outline-none transition focus-visible:border-cyan-300/60 focus-visible:ring-2 focus-visible:ring-cyan-300/20',
          error ? 'border-rose-400/60 focus-visible:border-rose-300/60 focus-visible:ring-rose-300/20' : '',
          className,
        )}
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
      {error || hint ? (
        <span id={messageId} className={cn('text-xs', error ? 'text-rose-200' : 'text-slate-400')}>
          {error ?? hint}
        </span>
      ) : null}
    </label>
  )
})
