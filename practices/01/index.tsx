import { cube } from 'geometry-processing'
import { Canvas } from '../components/canvas'

export function Practice01() {
  const args = Array.from(cube().args) as [number, number, number]
  return (
    <Canvas>
      <mesh>
        <boxGeometry args={args} />
        <meshPhongMaterial />
      </mesh>
    </Canvas>
  )
}
