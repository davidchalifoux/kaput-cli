const {expect, test} = require('@oclif/test')

describe('transfers:cancel', () => {
  test
  .stdout()
  .command(['transfers:cancel'])
  .it('runs hello', ctx => {
    expect(ctx.stdout).to.contain('hello world')
  })

  test
  .stdout()
  .command(['transfers:cancel', '--name', 'jeff'])
  .it('runs hello --name jeff', ctx => {
    expect(ctx.stdout).to.contain('hello jeff')
  })
})