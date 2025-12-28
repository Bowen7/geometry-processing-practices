import { Environment, OrbitControls } from '@react-three/drei'
import { Canvas } from '@react-three/fiber'
import init, { cube } from 'geometry-processing'
import { useWindowSize } from 'usehooks-ts'

// eslint-disable-next-line antfu/no-top-level-await
await init()

export function Practice01() {
  const { width, height } = useWindowSize()
  const args = Array.from(cube().args) as [number, number, number]
  return (
    <Canvas style={{ width, height }}>
      <mesh>
        <boxGeometry args={args} />
        <meshPhongMaterial />
      </mesh>
      <ambientLight intensity={0.1} />
      <directionalLight position={[0, 0, 5]} color="red" />
      <Environment preset="sunset" environmentIntensity={0.5} />
      <OrbitControls />
    </Canvas>
  )
}
