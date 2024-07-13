// https://on.cypress.io/api
describe('check home', () => {
  it('add a collection from name', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="add-collection-button"]').click()
    cy.get('[data-cy="add-collection-dialog"]').should('be.visible')
    cy.get('[data-cy="add-collection-dialog"]').contains('Enter the collection name').should('be.visible')
    cy.get('[data-cy="add-collection-dialog"]').contains('Collection name :').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-collection-name-field"]').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-from-playlist-check"]').should('not.be.checked')
    cy.get('[data-cy="add-collection-dialog"]').contains('From playlist :').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-from-playlist-field"]').should('not.exist')
    cy.get('[data-cy="add-collection-dialog-collection-name-field"]').type('test_new')
    cy.get('[data-cy="add-collection-dialog-actions"]').within(() => {
      cy.get('[data-mdc-dialog-action="accept"]').click()
    })
    cy.wait(500)
    cy.get('[data-cy="collection-list"]').click()
    cy.get('li').contains('test_new').click()
    cy.get('[data-cy="collection-data-url"]').contains('test_new').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_track_3').should('exist')
  })

  it('add a collection from playlist', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="add-collection-button"]').click()
    cy.get('[data-cy="add-collection-dialog"]').should('be.visible')
    cy.get('[data-cy="add-collection-dialog"]').contains('Enter the collection name').should('be.visible')
    cy.get('[data-cy="add-collection-dialog"]').contains('Collection name :').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-collection-name-field"]').should('be.visible')
    cy.get('[data-cy="add-collection-dialog"]').contains('From playlist :').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-from-playlist-field"]').should('not.exist')
    cy.get('[data-cy="add-collection-dialog-from-playlist-check"]').within(() => {
      cy.get('input').check()
    })
    cy.get('[data-cy="add-collection-dialog"]').contains('Collection name :').should('not.exist')
    cy.get('[data-cy="add-collection-dialog-collection-name-field"]').should('not.exist')
    cy.get('[data-cy="add-collection-dialog"]').contains('From playlist :').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-from-playlist-field"]').should('be.visible')
    cy.get('[data-cy="add-collection-dialog-from-playlist-field"]').type('https://www.deezer.com/fr/playlist/123456')
    cy.get('[data-cy="add-collection-dialog-actions"]').within(() => {
      cy.get('[data-mdc-dialog-action="accept"]').click()
    })
    cy.wait(500)
    cy.get('[data-cy="add-collection-dialog"]').should('not.be.visible')
    cy.get('[data-cy="collection-list"]').click()
    cy.get('li').contains('test_playlist_1').click()
    cy.get('[data-cy="collection-data-url"]').contains('test_playlist_1').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_track_1').should('exist')
    cy.get('[data-cy="track-list"]').contains('test_track_2').should('exist')
  })

  it('add collection dependency select', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="add-child-collection-button"]').click()
    cy.get('[data-cy="add-child-collection-dialog"]').should('be.visible')
    cy.get('[data-cy="add-child-collection-dialog-from-playlist-check"]').should('not.be.checked')
    cy.get('[data-cy="add-child-collection-dialog-select"]').should('be.visible')
    cy.get('[data-cy="add-child-collection-dialog-url-field"]').should('not.exist')
    cy.get('[data-cy="add-child-collection-dialog-select"]').click()
    cy.get('[data-cy="add-child-collection-dialog-select"]').within(() => {
      cy.get('li').contains('test_playlist_1').click()
    })
    cy.get('[data-cy="add-child-collection-dialog-actions"]').within(() => {
      cy.get('[data-mdc-dialog-action="accept"]').click()
    })
    cy.wait(500)
    cy.get('[data-cy="add-child-collection-dialog"]').should('not.be.visible')
    cy.get('[data-cy="children-collection-list"]').contains('test_playlist_1')
  })

  it('remove collection', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="collection-list"]').click()
    cy.get('li').contains('test_playlist_1').click()
    cy.get('[data-cy="remove-collection-button"]').click()
    cy.get('[data-cy="remove-collection-dialog"]').should('be.visible')
    cy.get('[data-cy="remove-collection-dialog-actions"]').within(() => {
      cy.get('[data-mdc-dialog-action="accept"]').click()
    })
    cy.wait(500)
    cy.get('[data-cy="remove-collection-dialog"]').should('not.be.visible')
    cy.get('[data-cy="children-collection-list"]').contains('test_playlist_1').should("not.exist")
    cy.get('[data-cy="collection-list"]').contains('test_playlist_1').should('not.exist')
  })

  it('add collection dependency from url', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="add-child-collection-button"]').click()
    cy.get('[data-cy="add-child-collection-dialog"]').should('be.visible')
    cy.get('[data-cy="add-child-collection-dialog-from-playlist-check"]').should('not.be.checked')
    cy.get('[data-cy="add-child-collection-dialog-select"]').should('be.visible')
    cy.get('[data-cy="add-child-collection-dialog-url-field"]').should('not.exist')
    cy.get('[data-cy="add-child-collection-dialog-from-playlist-check"]').within(() => {
      cy.get('input').check()
    })
    cy.get('[data-cy="add-child-collection-dialog-url-field"]').should('be.visible')
    cy.get('[data-cy="add-child-collection-dialog-select"]').should('not.exist')
    cy.get('[data-cy="add-child-collection-dialog-url-field"]').type('https://www.deezer.com/fr/playlist/123456')

    cy.get('[data-cy="add-child-collection-dialog-actions"]').within(() => {
      cy.get('[data-mdc-dialog-action="accept"]').click()
    })
    cy.wait(1000)
    cy.get('[data-cy="add-child-collection-dialog"]').should('not.be.visible')
    cy.get('[data-cy="children-collection-list"]').contains('test_playlist_1')
  })

  it('remove child collection', () => {
    cy.visit('http://localhost:5173')
    cy.get('[data-cy="remove-child-collection-button"]').click()
    cy.get('[data-cy="remove-child-collection-dialog"]').should('be.visible')
    cy.get('[data-cy="remove-child-collection-dialog-actions"]').within(() => {
      cy.get('[data-mdc-dialog-action="accept"]').click()
    })
    cy.wait(500)
    cy.get('[data-cy="remove-child-collection-dialog"]').should('not.be.visible')
    cy.get('[data-cy="children-collection-list"]').contains('test_playlist_1').should("not.exist")
    cy.get('[data-cy="add-collection-dialog"]').should('not.be.visible')
    cy.get('[data-cy="collection-list"]').click()
    cy.get('li').contains('test_playlist_1').should('exist')
  })

})
  
