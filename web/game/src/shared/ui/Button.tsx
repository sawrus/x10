import type { ButtonHTMLAttributes, PropsWithChildren } from 'react'

import { cn } from '../lib/cn'

type ButtonVariant = 'ghost' | 'primary' | 'secondary'
type ButtonSize = 'md' | 'sm'

export type ButtonProps = PropsWithChildren<
  ButtonHTMLAttributes<HTMLButtonElement> & {
    size?: ButtonSize
    variant?: ButtonVariant
  }
>

const variantClasses: Record<ButtonVariant, string> = {
  ghost: 'border border-white/10 bg-transparent text-slate-200 hover:border-cyan-300/40 hover:text-white',
  primary: 'bg-cyan-300 text-slate-950 hover:bg-cyan-200',
  secondary: 'border border-white/10 bg-slate-900/80 text-slate-100 hover:border-cyan-300/40 hover:bg-slate-800',
}

const sizeClasses: Record<ButtonSize, string> = {
  md: 'px-4 py-2.5 text-sm',
  sm: 'px-3 py-2 text-xs',
}

export function Button({ children, className, size = 'md', type = 'button', variant = 'primary', ...props }: ButtonProps) {
  return (
    <button
      {...props}
      type={type}
      className={cn(
        'inline-flex items-center justify-center rounded-full font-semibold transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-300/80 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-950 disabled:cursor-not-allowed disabled:opacity-60',
        variantClasses[variant],
        sizeClasses[size],
        className,
      )}
    >
      {children}
    </button>
  )
}
