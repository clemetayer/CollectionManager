<script lang="ts">
import { defineComponent } from 'vue';
import { CollectionService } from '../service/collection.service';
import type Collection from '../models/collection.model';

    type SelectOption = {
        label : string,
        value : string,
        disabled ?: boolean
    }
    export default defineComponent({
        name:"collection-select",
        data() {
            return {
                options: [] as SelectOption[],
                selectedOption: {} as SelectOption
            }
        },
        methods: {
            retrieveCollections() {
                let collectionService = new CollectionService();
                collectionService
                    .listCollections()
                    .then((collections) => {
                        this.options = collections.map((collection) => this.collectionToSelectOption(collection));
                        this.selectedOption = this.options[0]
                        console.log("options = " + this.options);
                    })
            },
            collectionToSelectOption(collection : Collection) : SelectOption {
                return {
                    label:collection.name,
                    value:collection.name
                };
            },
            onSelectCollection(pSelectedOption : any) {
                this.selectedOption = pSelectedOption;
                console.log(this.selectedOption.value);
            },
            addCollection() {
                console.log("add collection");
            }
        },
        mounted() {
            this.retrieveCollections();
        }
    });
</script>

<template>
    <center>
        <v-container>
            <ui-select v-model="selectedOption" :options="options" @selected="onSelectCollection($event)"></ui-select>
            <ui-icon-button icon="add" @click="addCollection()"></ui-icon-button>
        </v-container>
    </center>
</template>
