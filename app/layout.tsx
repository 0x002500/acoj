export const metadata = {
  title: 'acoj',
  description: 'A OJ powered by Rust and Next.js',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
