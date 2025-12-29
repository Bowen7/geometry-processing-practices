import { Spinner } from '@/components/ui/spinner'

export function Loading() {
  return (
    <div className="fixed top-0 left-0 w-full h-full flex items-center justify-center">
      <Spinner />
    </div>
  )
}
