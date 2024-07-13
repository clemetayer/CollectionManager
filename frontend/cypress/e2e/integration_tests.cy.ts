// https://on.cypress.io/api
describe('check home', () => {
  it('check home valid', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="collection-list"]').contains('test_playlist_1').should('exist')
    cy.get('[data-cy="collection-list"]').contains('test_new').should('exist')
    cy.get('[data-cy="collection-data-url"]').contains('test_playlist_1').should('exist')
    cy.get('[data-cy="children-collection-list"]').contains('test_new').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_track_1').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_artist_1').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_track_2').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_artist_2').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_artist_3').should('not.exist')
  })

  it('check change collection view', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="collection-list"]').click()
    cy.get('li').contains('test_new').click()
    cy.wait(500)
    cy.get('[data-cy="collection-data-url"]').contains('test_new').should('exist')
    cy.get('[data-cy="collection-data-url"]').contains('test_playlist_1').should('not.exist')
    cy.get('[data-cy="track-list"]').contains('test_track_3').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_track_1').should('not.exist')
    cy.get('[data-cy="track-list"]').contains('test_track_2').should('not.exist')

  })
})
  
