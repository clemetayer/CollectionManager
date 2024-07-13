<script lang="ts">
import { defineComponent } from 'vue';
import { CollectionService } from '../service/collection.service';
import CollectionData from './CollectionData.vue';
import AddCollectionDialog from './dialogs/AddCollectionDialog.vue';
import { type Collection } from "../models/collection.model"
import { type SelectOption } from "../models/balmui.model"

    let collectionService = new CollectionService();

    export default defineComponent({
        name:"collection-select",
        components: {
            CollectionData,
            AddCollectionDialog
        },
        data() {
            return {
                options: [] as SelectOption[],
                selectedOption: '',
                collectionName: "",
                fromPlaylistChecked: false,
                fromPlaylist: ""
            }
        },
        methods: {
            retrieveCollections() {
                collectionService
                    .listCollections()
                    .then((collections) => {
                        this.options = collections.map((collection) => this.collectionToSelectOption(collection));
                        if(this.options.length > 0) {
                            this.selectedOption = this.options[0].value;
                            this.$refs.collectionDataRef.displayCollection(this.selectedOption);
                        }
                        console.log("options = " + this.options);
                    })
            },
            collectionToSelectOption(collection : Collection) : SelectOption {
                return {
                    label:collection.name,
                    value:collection.deezer_id
                };
            },
            onSelectCollection(pSelectedOption : any) {
                this.selectedOption = pSelectedOption.value;
                this.$refs.collectionDataRef.displayCollection(pSelectedOption.value);
                this.$refs.collectionDataRef.collections = this.options;
                console.log(this.selectedOption);
            },
            addCollection() {
                this.$refs.addCollectionDialogRef.openDialog()
            },
            updateAllCollections() {
                collectionService.updateAllCollections();
            },
            refreshData() {
                this.retrieveCollections();
            }
        },
        mounted() {
            this.retrieveCollections();
        }
    });
</script>

<template>
    <div>
        <div style="text-align:center">
            <v-container>
                <ui-select v-model="selectedOption" :options="options" @selected="onSelectCollection($event)" data-cy="collection-list"></ui-select>
                <ui-icon-button icon="add" @click="addCollection()" data-cy="add-collection-button"></ui-icon-button>
            </v-container>
            <br/>
            <ui-button @click="updateAllCollections">Update all collections</ui-button>
            <CollectionData ref="collectionDataRef" @refresh-data="refreshData"/>
        </div>
        <AddCollectionDialog ref="addCollectionDialogRef" @refresh-data="refreshData"/>
    </div>
</template>
