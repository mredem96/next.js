import { z } from 'next/dist/compiled/zod'

/**
 * The schema for the page segment config.
 *
 * @internal - required to exclude zod types from the build
 */
export const PagesSegmentConfigSchema = z.object({
  /**
   * The runtime to use for the page.
   */
  runtime: z.enum(['edge', 'experimental-edge', 'nodejs']).optional(),

  /**
   * The maximum duration for the page render.
   */
  maxDuration: z.number().optional(),

  /**
   * The exported config object for the page.
   */
  config: z
    .object({
      /**
       * Enables AMP for the page.
       */
      amp: z.union([z.boolean(), z.literal('hybrid')]).optional(),

      /**
       * The runtime to use for the page.
       */
      runtime: z.enum(['edge', 'experimental-edge', 'nodejs']).optional(),
    })
    .optional(),
})

/**
 * The keys of the configuration for a page.
 *
 * @internal - required to exclude zod types from the build
 */
export const PagesSegmentConfigSchemaKeys =
  PagesSegmentConfigSchema.keyof().options

export type PagesSegmentConfigConfig = {
  /**
   * Enables AMP for the page.
   */
  amp?: boolean | 'hybrid'

  /**
   * The runtime to use for the page.
   */
  runtime?: 'edge' | 'experimental-edge' | 'nodejs'

  /**
   * The preferred region for the page.
   */
  regions?: string[]
}

export type PagesSegmentConfig = {
  /**
   * The runtime to use for the page.
   */
  runtime?: 'edge' | 'experimental-edge' | 'nodejs'

  /**
   * The maximum duration for the page render.
   */
  maxDuration?: number

  /**
   * The exported config object for the page.
   */
  config?: PagesSegmentConfigConfig
}
