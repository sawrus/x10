import { useEffect, useState } from 'react'

import { useAppStore } from '../../../app/store'
import type { Profile, ProfilePhotoSummary } from '../../../entities/profile'
import {
  getProfilePhotoBlob,
  useDeleteProfilePhotoApiMutation,
  useProfilePhotosApiQuery,
  useSelectProfilePhotoApiMutation,
  useUploadProfilePhotoApiMutation,
} from '../../../shared/api'

export function useCurrentProfilePhotos(profileId: string, enabled = true) {
  const actorId = useAppStore((state) => state.profileId)

  return useProfilePhotosApiQuery(profileId, {
    enabled: enabled && Boolean(actorId),
    request: actorId ? { actorId } : undefined,
    retry: false,
  })
}

export function useUploadCurrentProfilePhoto(profileId: string) {
  const actorId = useAppStore((state) => state.profileId)

  return useUploadProfilePhotoApiMutation(profileId, {
    request: actorId ? { actorId } : undefined,
  })
}

export function useDeleteCurrentProfilePhoto(profileId: string) {
  const actorId = useAppStore((state) => state.profileId)

  return useDeleteProfilePhotoApiMutation(profileId, {
    request: actorId ? { actorId } : undefined,
  })
}

export function useSelectCurrentProfilePhoto(profileId: string) {
  const actorId = useAppStore((state) => state.profileId)

  return useSelectProfilePhotoApiMutation(profileId, {
    request: actorId ? { actorId } : undefined,
  })
}

export function useProfilePhotoObjectUrl(profileId: string, photo: ProfilePhotoSummary | null | undefined) {
  const actorId = useAppStore((state) => state.profileId)
  const [url, setUrl] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!photo || !actorId) {
      setUrl(null)
      setError(null)
      return undefined
    }

    let active = true
    let objectUrl: string | null = null

    void getProfilePhotoBlob(photo.id, profileId, { actorId })
      .then((blob) => {
        if (!active) {
          return
        }

        objectUrl = URL.createObjectURL(blob)
        setUrl(objectUrl)
        setError(null)
      })
      .catch((reason) => {
        if (!active) {
          return
        }

        setUrl(null)
        setError(reason instanceof Error ? reason.message : 'Failed to load photo preview.')
      })

    return () => {
      active = false
      if (objectUrl) {
        URL.revokeObjectURL(objectUrl)
      }
    }
  }, [actorId, photo, profileId])

  return {
    error,
    url,
  }
}

export function resolveCurrentProfilePhoto(profile: Profile, photos: ProfilePhotoSummary[]): ProfilePhotoSummary | null {
  return photos.find((photo) => photo.id === profile.current_photo_id) ?? null
}
