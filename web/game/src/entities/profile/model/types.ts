import type { ProfileId } from '../../../shared/model'

export type Profile = {
  readonly birth_date: string
  readonly current_photo_id: string | null
  readonly email: string | null
  readonly full_name: string
  readonly id: ProfileId
  readonly occupation: string
  readonly telegram: string | null
  readonly timezone: string
}

export type CreateProfileDto = {
  readonly birth_date: string
  readonly email?: string | null
  readonly full_name: string
  readonly occupation: string
  readonly telegram?: string | null
  readonly timezone: string
}

export type UpdateProfileDto = {
  readonly birth_date?: string
  readonly email?: string | null
  readonly full_name?: string
  readonly occupation?: string
  readonly telegram?: string | null
  readonly timezone?: string
}

export type CreateProfilePayload = CreateProfileDto
export type UpdateProfilePayload = UpdateProfileDto

export type ProfilePhotoSummary = {
  readonly content_url: string
  readonly id: string
  readonly mime_type: string
  readonly original_name: string
  readonly size_bytes: number
}

export type ProfileProgression = {
  readonly balance_score: number
  readonly current_level: string
  readonly current_level_id: string
}

export type ProfileDashboard = {
  readonly current: ProfileProgression
  readonly current_photo: ProfilePhotoSummary | null
  readonly profile: Profile
}

export type ProfileFormValues = {
  readonly birthDate: string
  readonly email: string
  readonly fullName: string
  readonly occupation: string
  readonly telegram: string
  readonly timezone: string
}

export type ProfileFormErrors = Partial<Record<keyof ProfileFormValues, string>>
