import { Environment, OrbitControls } from '@react-three/drei'
import { Canvas as ThreeCanvas } from '@react-three/fiber'
import init from 'geometry-processing'
import { useWindowSize } from 'usehooks-ts'

// eslint-disable-next-line antfu/no-top-level-await
await init()

interface Props {
  children?: React.ReactNode
}
export function Canvas({ children }: Props) {
  const { width, height } = useWindowSize()

  return (
    <ThreeCanvas style={{ width, height }}>
      {children}
      <ambientLight intensity={0.1} />
      <directionalLight position={[0, 0, 5]} color="red" />
      <Environment preset="sunset" environmentIntensity={0.5} />
      <OrbitControls />
    </ThreeCanvas>
  )
}
