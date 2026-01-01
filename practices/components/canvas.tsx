import { OrbitControls } from '@react-three/drei'
import { Canvas as ThreeCanvas } from '@react-three/fiber'
import init from 'geometry-processing'
import { useWindowSize } from 'usehooks-ts'

// eslint-disable-next-line antfu/no-top-level-await
await init()

interface Props {
  children?: React.ReactNode
  camera?: [number, number, number]
  enableRotate?: boolean
}
export function Canvas({ children, camera = [0, 0, 5], enableRotate }: Props) {
  const { width, height } = useWindowSize()

  return (
    <ThreeCanvas style={{ width, height }} camera={{ position: camera }}>
      {children}
      <ambientLight intensity={0.1} />
      <directionalLight position={[0, 0, 5]} color="#fff" />
      <OrbitControls enableRotate={enableRotate} />
    </ThreeCanvas>
  )
}
