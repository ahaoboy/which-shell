import { spawnSync } from 'child_process'

export const SHELLS = [
  'fish',
  'zsh',
  'bash',
  'powershell',
  'cmd',
  'pwsh',
  'nu',
  'dash',
  'ksh',
  'tcsh',
  'csh',
  'sh',
] as const

export type SHELL = (typeof SHELLS)[number]
export type ShellVersion = {
  shell: SHELL
  version?: string
}

function getPpidUnix(pid: number): { name: string; pid: number } | undefined {
  const s = spawnSync('ps', ['-p', pid.toString(), '-o', 'comm,ppid'])
  if (!s.stdout || s.status !== 0) {
    return
  }
  const stdout = s.stdout.toString().trim()
  const v = stdout?.replaceAll('\r\n', '\n').split('\n').map((i) => i.trim())[1]
  if (!v) {
    return
  }
  const re = /^(\S+)\s+(\d+)$/
  const ret = v.match(re)
  if (!ret) {
    return
  }
  const name = getFilename(ret[1].trim())
  const ppid = +ret[2].trim()
  if (name && Number.isInteger(ppid)) {
    return { pid: ppid, name }
  }
  return
}

function getPpidWindows(
  pid: number,
): { name: string; pid: number } | undefined {
  const cmd =
    `$p = (Get-CimInstance -ClassName Win32_Process -Filter "ProcessId = ${pid}"); Write-Output $p.ParentProcessId $p.Name`
  const s = spawnSync('powershell', ['-c', cmd])
  if (!s.stdout || s.status !== 0) {
    return
  }
  const stdout = s.stdout.toString().trim()
  const v = stdout?.replaceAll('\r\n', '\n').split('\n').map((i) => i.trim())
  const ppid = +v[0]
  const name = getFilename(v[1])
  if (!Number.isInteger(ppid) || !name) {
    return
  }
  return { pid: ppid, name }
}

const getPpid = process.platform === 'win32' ? getPpidWindows : getPpidUnix

function isShell(sh: string): sh is SHELL {
  return SHELLS.includes(sh as SHELL)
}

function getShellVersion(sh: string): string | undefined {
  const cmd = {
    'powershell': ['-c', "$PSVersionTable.PSVersion -replace '\\D', '.'"],
    'ksh': ['-c', 'echo $KSH_VERSION'],
  }[sh] ?? ['--version']

  const s = spawnSync(sh, cmd)
  const stdout = s.stdout.toString().trim()
  if (s.status !== 0 || !stdout.length) {
    return
  }

  switch (sh) {
    case 'fish': {
      return stdout.slice(14).trim()
    }
    case 'pwsh': {
      return stdout.slice(11).trim()
    }
    case 'bash': {
      const re = /([0-9]+).([0-9]+).([0-9]+)/
      const m = stdout.match(re)
      if (m) {
        return m.slice(1, 3).join('.')
      }
    }
    case 'cmd': {
      return stdout.replaceAll('\r\n', '\n').split('\n').at(0)?.split(' ').at(
        -1,
      )?.split(']').at(0)?.trim()
    }
    case 'ksh': {
      return stdout.split('/').at(1)?.split(' ').at(1)?.trim()
    }
    case 'zsh': {
      return stdout.split(' ').at(1)?.trim()
    }
    case 'tcsh': {
      return stdout.split(' ').at(1)?.trim()
    }
    case 'powershell':
    case 'nu': {
      return stdout
    }
  }
  return undefined
}

function getFilename(s: string): string | undefined {
  return s.replaceAll('\\', '/').split('/').at(-1)?.split('.').at(0)?.trim()
}

function guessShell(shell: string): ShellVersion | undefined {
  if (isShell(shell)) {
    return { shell, version: getShellVersion(shell) }
  }
  return
}

export function whichShell(): ShellVersion | undefined {
  let pid = process.pid
  let name = ''
  while (pid) {
    const pp = getPpid(pid)
    if (!pp) {
      break
    }
    pid = pp.pid
    name = pp.name
    const sh = guessShell(name)
    if (sh) {
      return sh
    }
  }
}
