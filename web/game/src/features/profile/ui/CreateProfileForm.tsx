import { useState, type FormEvent } from 'react'

import type { CreateProfileDto } from '../../../entities/profile'
import { Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input } from '../../../shared/ui'

type CreateProfileFormProps = {
  isSaving: boolean
  onSubmit: (payload: CreateProfileDto) => Promise<unknown>
  submitError?: Error | null
}

type DraftState = {
  birthDate: string
  email: string
  fullName: string
  occupation: string
  telegram: string
  timezone: string
}

type DraftErrors = Partial<Record<keyof DraftState, string>>

function createInitialState(): DraftState {
  return {
    birthDate: '',
    email: '',
    fullName: '',
    occupation: '',
    telegram: '',
    timezone: Intl.DateTimeFormat().resolvedOptions().timeZone || 'Europe/Samara',
  }
}

function validate(values: DraftState): DraftErrors {
  const errors: DraftErrors = {}

  if (!values.fullName.trim()) {
    errors.fullName = 'Full name is required.'
  }
  if (!values.birthDate.trim()) {
    errors.birthDate = 'Birth date is required.'
  }
  if (!values.occupation.trim()) {
    errors.occupation = 'Occupation is required.'
  }
  if (!values.timezone.trim()) {
    errors.timezone = 'Timezone is required.'
  }

  return errors
}

export function CreateProfileForm({ isSaving, onSubmit, submitError }: CreateProfileFormProps) {
  const [values, setValues] = useState<DraftState>(createInitialState)
  const [errors, setErrors] = useState<DraftErrors>({})

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    const nextErrors = validate(values)
    setErrors(nextErrors)

    if (Object.keys(nextErrors).length) {
      return
    }

    await onSubmit({
      birth_date: values.birthDate.trim(),
      email: values.email.trim() || null,
      full_name: values.fullName.trim(),
      occupation: values.occupation.trim(),
      telegram: values.telegram.trim() || null,
      timezone: values.timezone.trim(),
    })
  }

  function update<TKey extends keyof DraftState>(field: TKey, value: DraftState[TKey]) {
    const nextValues = {
      ...values,
      [field]: value,
    }

    setValues(nextValues)
    if (Object.keys(errors).length) {
      setErrors(validate(nextValues))
    }
  }

  return (
    <Card className="rounded-[2rem] bg-slate-900/70">
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Create character</p>
        <CardTitle className="mt-3 text-3xl">Build the first hero sheet</CardTitle>
        <CardDescription className="mt-4">
          Start with a grounded profile. This gives the next quest and progression steps a real character to work with.
        </CardDescription>
      </CardHeader>

      <CardContent>
        <form className="space-y-4" onSubmit={handleSubmit}>
          <div className="grid gap-4 sm:grid-cols-2">
            <Input label="Full name" value={values.fullName} error={errors.fullName} onChange={(event) => update('fullName', event.target.value)} />
            <Input label="Occupation" value={values.occupation} error={errors.occupation} onChange={(event) => update('occupation', event.target.value)} />
            <Input label="Birth date" type="date" value={values.birthDate} error={errors.birthDate} onChange={(event) => update('birthDate', event.target.value)} />
            <Input
              label="Timezone"
              value={values.timezone}
              error={errors.timezone}
              hint="Use an IANA timezone like Europe/Samara."
              onChange={(event) => update('timezone', event.target.value)}
            />
            <Input label="Telegram" value={values.telegram} hint="Optional." onChange={(event) => update('telegram', event.target.value)} />
            <Input label="Email" type="email" value={values.email} hint="Optional." onChange={(event) => update('email', event.target.value)} />
          </div>

          {submitError ? (
            <div role="alert" className="rounded-[1.5rem] border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">
              Failed to create profile: {submitError.message}
            </div>
          ) : null}

          <CardFooter className="mt-0 px-0 pb-0">
            <Button disabled={isSaving} type="submit">
              {isSaving ? 'Creating...' : 'Create character'}
            </Button>
          </CardFooter>
        </form>
      </CardContent>
    </Card>
  )
}
