export const dynamic = "force-dynamic";

export async function GET() {
  return new Response("Hello World!", {
    status: 200,
  });
}
