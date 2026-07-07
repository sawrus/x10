import { useEffect, useId } from 'react'
import type { PropsWithChildren, ReactNode } from 'react'
import { createPortal } from 'react-dom'

import { cn } from '../lib/cn'
import { Button } from './Button'

export type ModalProps = PropsWithChildren<{
  description?: string
  footer?: ReactNode
  onClose: () => void
  open: boolean
  title: string
}>

export function Modal({ children, description, footer, onClose, open, title }: ModalProps) {
  const titleId = useId()
  const descriptionId = useId()

  useEffect(() => {
    if (!open) {
      return undefined
    }

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        onClose()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [onClose, open])

  if (!open || typeof document === 'undefined') {
    return null
  }

  return createPortal(
    <div
      aria-labelledby={titleId}
      aria-describedby={description ? descriptionId : undefined}
      aria-modal="true"
      className="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/80 px-4 py-8 backdrop-blur-sm"
      role="dialog"
      onClick={(event) => {
        if (event.target === event.currentTarget) {
          onClose()
        }
      }}
    >
      <div className="w-full max-w-lg rounded-[2rem] border border-white/10 bg-slate-900 p-6 shadow-2xl shadow-slate-950/60">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h2 id={titleId} className="text-2xl font-black tracking-tight text-slate-50">
              {title}
            </h2>
            {description ? (
              <p id={descriptionId} className="mt-2 text-sm text-slate-300">
                {description}
              </p>
            ) : null}
          </div>
          <Button aria-label="Close modal" size="sm" variant="ghost" onClick={onClose}>
            Close
          </Button>
        </div>

        <div className="mt-6 text-sm text-slate-200">{children}</div>

        <div className={cn('mt-6 flex flex-wrap justify-end gap-3', footer ? '' : 'hidden')}>
          {footer}
        </div>
      </div>
    </div>,
    document.body,
  )
}
