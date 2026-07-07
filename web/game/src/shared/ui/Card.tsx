import type { HTMLAttributes, PropsWithChildren } from 'react'

import { cn } from '../lib/cn'

export function Card({ children, className, ...props }: PropsWithChildren<HTMLAttributes<HTMLDivElement>>) {
  return (
    <div
      {...props}
      className={cn(
        'rounded-[2rem] border border-white/10 bg-slate-900/70 p-6 shadow-xl shadow-slate-950/30 backdrop-blur',
        className,
      )}
    >
      {children}
    </div>
  )
}

export function CardHeader({ children, className, ...props }: PropsWithChildren<HTMLAttributes<HTMLDivElement>>) {
  return (
    <div {...props} className={cn('flex flex-col gap-3', className)}>
      {children}
    </div>
  )
}

export function CardTitle({ children, className, ...props }: PropsWithChildren<HTMLAttributes<HTMLHeadingElement>>) {
  return (
    <h2 {...props} className={cn('text-3xl font-black tracking-tight', className)}>
      {children}
    </h2>
  )
}

export function CardDescription({ children, className, ...props }: PropsWithChildren<HTMLAttributes<HTMLParagraphElement>>) {
  return (
    <p {...props} className={cn('max-w-2xl text-base text-slate-300', className)}>
      {children}
    </p>
  )
}

export function CardContent({ children, className, ...props }: PropsWithChildren<HTMLAttributes<HTMLDivElement>>) {
  return (
    <div {...props} className={cn('mt-6', className)}>
      {children}
    </div>
  )
}

export function CardFooter({ children, className, ...props }: PropsWithChildren<HTMLAttributes<HTMLDivElement>>) {
  return (
    <div {...props} className={cn('mt-6 flex flex-wrap items-center gap-3', className)}>
      {children}
    </div>
  )
}
