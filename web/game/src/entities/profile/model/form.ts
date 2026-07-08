import type { Profile, ProfileFormErrors, ProfileFormValues, UpdateProfileDto } from './types'

export function createProfileFormValues(profile: Profile): ProfileFormValues {
  return {
    fullName: profile.full_name,
    birthDate: profile.birth_date,
    occupation: profile.occupation,
    telegram: profile.telegram ?? '',
    email: profile.email ?? '',
    timezone: profile.timezone,
  }
}

export function validateProfileForm(values: ProfileFormValues): ProfileFormErrors {
  const errors: ProfileFormErrors = {}

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

export function hasProfileFormErrors(errors: ProfileFormErrors): boolean {
  return Object.keys(errors).length > 0
}

export function buildProfileUpdatePayload(values: ProfileFormValues): UpdateProfileDto {
  return {
    full_name: values.fullName.trim(),
    birth_date: values.birthDate.trim(),
    occupation: values.occupation.trim(),
    telegram: normalizeOptionalText(values.telegram),
    email: normalizeOptionalText(values.email),
    timezone: values.timezone.trim(),
  }
}

export function areProfileFormValuesEqual(left: ProfileFormValues, right: ProfileFormValues): boolean {
  return (
    left.fullName === right.fullName &&
    left.birthDate === right.birthDate &&
    left.occupation === right.occupation &&
    left.telegram === right.telegram &&
    left.email === right.email &&
    left.timezone === right.timezone
  )
}

function normalizeOptionalText(value: string): string | null {
  const normalizedValue = value.trim()

  return normalizedValue ? normalizedValue : null
}
