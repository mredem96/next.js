import picomatch from 'next/dist/compiled/picomatch'
import { z } from 'next/dist/compiled/zod'

const RouteHasSchema = z.union([
  z.object({
    type: z.enum(['header', 'query', 'cookie']),
    key: z.string(),
    value: z.string().optional(),
  }),
  z.object({
    type: z.literal('host'),
    key: z.undefined().optional(),
    value: z.string(),
  }),
])

const GlobSchema = z.string().superRefine((val, ctx) => {
  try {
    picomatch(val)
  } catch (err: any) {
    ctx.addIssue({
      code: z.ZodIssueCode.custom,
      message: `Invalid glob pattern '${val}': ${err.message}`,
    })
  }
})

/**
 * @internal - required to exclude zod types from the build
 */
export const MiddlewareConfigInputSchema = z.object({
  /**
   * The matcher for the middleware.
   */
  matcher: z
    .union([
      z.string(),
      z.array(z.string()),
      z.array(
        z.object({
          locale: z.literal(false).optional(),
          has: z.array(RouteHasSchema).optional(),
          missing: z.array(RouteHasSchema).optional(),
          source: z.string(),
        })
      ),
    ])
    .optional(),

  /**
   * The regions that the middleware should run in.
   */
  regions: z.union([z.string(), z.array(z.string())]).optional(),

  unstable_allowDynamic: z.union([GlobSchema, z.array(GlobSchema)]).optional(),
  unstable_allowDynamicGlobs: z.array(GlobSchema).optional(),
})

/**
 * The keys of the configuration for a middleware.
 *
 * @internal - required to exclude zod types from the build
 */
export const MiddlewareConfigInputSchemaKeys =
  MiddlewareConfigInputSchema.keyof().options
