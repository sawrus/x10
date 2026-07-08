import { useEffect, useMemo, useState, type FormEvent } from 'react'

import { isApiError } from '../../../shared/api'
import { Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input } from '../../../shared/ui'

import {
  areProfileFormValuesEqual,
  buildProfileUpdatePayload,
  createProfileFormValues,
  hasProfileFormErrors,
  validateProfileForm,
} from '../model/form'
import type { Profile, ProfileFormValues, UpdateProfileDto } from '../model/types'

type ProfileEditorFormProps = {
  isSaving: boolean
  onSubmit: (payload: UpdateProfileDto) => Promise<unknown>
  profile: Profile
  submitError?: unknown
}

export function ProfileEditorForm({ isSaving, onSubmit, profile, submitError }: ProfileEditorFormProps) {
  const baselineValues = useMemo(() => createProfileFormValues(profile), [profile])
  const [values, setValues] = useState<ProfileFormValues>(baselineValues)
  const [errors, setErrors] = useState(() => validateProfileForm(baselineValues))

  useEffect(() => {
    setValues(baselineValues)
    setErrors(validateProfileForm(baselineValues))
  }, [baselineValues])

  const isDirty = !areProfileFormValuesEqual(values, baselineValues)
  const isDisabled = isSaving || !isDirty || hasProfileFormErrors(errors)

  const submitErrorMessage = resolveSubmitErrorMessage(submitError)

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()

    const nextErrors = validateProfileForm(values)
    setErrors(nextErrors)

    if (hasProfileFormErrors(nextErrors)) {
      return
    }

    await onSubmit(buildProfileUpdatePayload(values))
  }

  function updateField<TKey extends keyof ProfileFormValues>(field: TKey, value: ProfileFormValues[TKey]) {
    const nextValues = {
      ...values,
      [field]: value,
    }

    setValues(nextValues)
    setErrors(validateProfileForm(nextValues))
  }

  return (
    <Card className="h-full rounded-[2rem] bg-slate-900/70">
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Profile editor</p>
        <CardTitle className="mt-3 text-3xl">Edit hero basics</CardTitle>
        <CardDescription className="mt-4">
          Keep the core profile fields editable here. Avatar management lives рядом на этом же экране, но не смешивается с базовой формой.
        </CardDescription>
      </CardHeader>

      <CardContent>
        <form className="space-y-4" onSubmit={handleSubmit}>
          <div className="grid gap-4 sm:grid-cols-2">
            <Input
              label="Full name"
              value={values.fullName}
              onChange={(event) => updateField('fullName', event.target.value)}
              error={errors.fullName}
            />
            <Input
              label="Occupation"
              value={values.occupation}
              onChange={(event) => updateField('occupation', event.target.value)}
              error={errors.occupation}
            />
            <Input
              label="Birth date"
              type="date"
              value={values.birthDate}
              onChange={(event) => updateField('birthDate', event.target.value)}
              error={errors.birthDate}
            />
            <Input
              label="Timezone"
              value={values.timezone}
              onChange={(event) => updateField('timezone', event.target.value)}
              error={errors.timezone}
              hint="Use an IANA timezone like Europe/Samara."
            />
            <Input
              label="Telegram"
              value={values.telegram}
              onChange={(event) => updateField('telegram', event.target.value)}
              hint="Optional. Leave blank to clear."
            />
            <Input
              label="Email"
              type="email"
              value={values.email}
              onChange={(event) => updateField('email', event.target.value)}
              hint="Optional. Leave blank to clear."
            />
          </div>

          {submitErrorMessage ? (
            <div role="alert" className="rounded-[1.5rem] border border-rose-400/30 bg-rose-400/10 px-4 py-3 text-sm text-rose-100">
              Failed to save profile changes: {submitErrorMessage}
            </div>
          ) : null}

          <CardFooter className="mt-0 px-0 pb-0">
            <Button disabled={isDisabled} type="submit">
              {isSaving ? 'Saving...' : isDirty ? 'Save changes' : 'Saved'}
            </Button>
            <Button
              disabled={isSaving || !isDirty}
              type="button"
              variant="secondary"
              onClick={() => {
                setValues(baselineValues)
                setErrors(validateProfileForm(baselineValues))
              }}
            >
              Reset
            </Button>
          </CardFooter>
        </form>
      </CardContent>
    </Card>
  )
}

function resolveSubmitErrorMessage(error: unknown): string | null {
  if (!error) {
    return null
  }

  if (isApiError(error)) {
    return error.details?.message ?? error.message
  }

  if (error instanceof Error) {
    return error.message
  }

  return 'Unknown save error.'
}
