import { useMagicKeys, useActiveElement, whenever, useEventListener } from '@vueuse/core'
import { computed } from 'vue'
import { useAudioStore } from '@/stores/audio'

export function useKeyboardShortcuts() {
  const store = useAudioStore()
  const keys = useMagicKeys()
  const activeElement = useActiveElement()
  
  const notUsingInput = computed(() => {
    const tag = activeElement.value?.tagName
    return tag !== 'INPUT' && tag !== 'TEXTAREA' && !activeElement.value?.isContentEditable
  })

  // Q/A, S, ArrowLeft -> previous page (ZQSD + WASD)
  whenever(keys['q']!, () => { if (notUsingInput.value) store.prevPage() })
  whenever(keys['a']!, () => { if (notUsingInput.value) store.prevPage() })
  whenever(keys['s']!, () => { if (notUsingInput.value) store.prevPage() })
  whenever(keys['ArrowLeft']!, () => { if (notUsingInput.value) store.prevPage() })

  // D, Z/W, ArrowRight -> next page (ZQSD + WASD)
  whenever(keys['d']!, () => { if (notUsingInput.value) store.nextPage() })
  whenever(keys['z']!, () => { if (notUsingInput.value) store.nextPage() })
  whenever(keys['w']!, () => { if (notUsingInput.value) store.nextPage() })
  whenever(keys['ArrowRight']!, () => { if (notUsingInput.value) store.nextPage() })

  // T -> toggle queue panel
  whenever(keys['t']!, () => { if (notUsingInput.value) store.isQueueExpanded = !store.isQueueExpanded })

  // Escape -> close queue panel
  whenever(keys['Escape']!, () => { if (store.isQueueExpanded) store.isQueueExpanded = false })

  // Ctrl+S -> stop all sounds
  whenever(keys['Ctrl+s']!, () => { if (notUsingInput.value) store.stopAll() })

  // M -> mute/unmute master volume
  let previousVolume = 1.0
  whenever(keys['m']!, () => {
    if (notUsingInput.value) {
      if (store.masterVolume > 0) {
        previousVolume = store.masterVolume
        store.setMasterVolume(0)
      } else {
        store.setMasterVolume(previousVolume)
      }
    }
  })

  // Space -> pause/resume all (with preventDefault to stop scroll)
  useEventListener('keydown', (e: KeyboardEvent) => {
    if (e.code === 'Space' && notUsingInput.value) {
      e.preventDefault()
      store.togglePauseAll()
    }
  })

  return { keys, notUsingInput }
}
