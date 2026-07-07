import { forwardRef, useId } from 'react'
import type { InputHTMLAttributes } from 'react'

import { cn } from '../lib/cn'

export type InputProps = InputHTMLAttributes<HTMLInputElement> & {
  error?: string
  hint?: string
  label?: string
}

export const Input = forwardRef<HTMLInputElement, InputProps>(function Input(
  { className, error, hint, id, label, ...props },
  ref,
) {
  const generatedId = useId()
  const inputId = id ?? generatedId
  const messageId = `${inputId}-message`

  return (
    <label className="flex w-full flex-col gap-2" htmlFor={inputId}>
      {label ? <span className="text-sm font-semibold text-slate-200">{label}</span> : null}
      <input
        {...props}
        ref={ref}
        id={inputId}
        aria-describedby={error || hint ? messageId : undefined}
        aria-invalid={error ? 'true' : undefined}
        className={cn(
          'w-full rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3 text-sm text-slate-100 outline-none transition placeholder:text-slate-500 focus-visible:border-cyan-300/60 focus-visible:ring-2 focus-visible:ring-cyan-300/20',
          error ? 'border-rose-400/60 focus-visible:border-rose-300/60 focus-visible:ring-rose-300/20' : '',
          className,
        )}
      />
      {error || hint ? (
        <span id={messageId} className={cn('text-xs', error ? 'text-rose-200' : 'text-slate-400')}>
          {error ?? hint}
        </span>
      ) : null}
    </label>
  )
})
