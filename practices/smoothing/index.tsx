import { useQuery } from '@tanstack/react-query'
import { wasm_smooth } from 'geometry-processing'
import { useControls } from 'leva'
import { useEffect, useMemo } from 'react'
import { Canvas } from '@/components/canvas'
import { Loading } from '@/components/loading'
import { buildGeometry } from '@/lib/geometry'
import { parseObj } from '@/lib/obj'

function Smoothing({ vertices: initialVertices, faces }: { vertices: Float32Array, faces: Uint32Array }) {
  const [{ lambda, steps }, set] = useControls(() => ({ lambda: {
    value: 0.8,
    min: 0,
    max: 1,
    step: 0.1,
  }, steps: {
    value: 0,
    step: 1,
    min: 0,
    max: 20,
  }, time: {
    value: '0ms',
    disabled: true,
    step: 1,
  } }))

  const { vertices, time } = useMemo(() => {
    // eslint-disable-next-line react-hooks/purity
    const start = performance.now()
    const { vertices } = wasm_smooth(initialVertices, faces, lambda, steps)
    // eslint-disable-next-line react-hooks/purity
    const end = performance.now()
    return { vertices, time: `${end - start}ms` }
  }, [initialVertices, faces, lambda, steps])

  useEffect(() => {
    set({ time })
  }, [time, set])
  const geometry = buildGeometry(vertices, faces)
  return (
    <mesh geometry={geometry} scale={0.01} rotation-x={-Math.PI / 2}>
      <meshPhongMaterial />
    </mesh>
  )
}

export function SmoothingPractice() {
  const { data } = useQuery({
    queryKey: ['max-noisy'],
    queryFn: () => fetch('data/max-noisy.obj').then(parseObj),
  })

  if (!data) {
    return <Loading />
  }

  return (
    <Canvas>
      <Smoothing vertices={data.vertices} faces={data.faces} />
      <axesHelper />
    </Canvas>
  )
}
